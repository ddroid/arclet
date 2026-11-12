use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize as NativePtySize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtySize {
    pub cols: u16,
    pub rows: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

impl Default for PtySize {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            pixel_width: 800,
            pixel_height: 480,
        }
    }
}

impl From<PtySize> for NativePtySize {
    fn from(val: PtySize) -> Self {
        NativePtySize {
            rows: val.rows.max(1),
            cols: val.cols.max(1),
            pixel_width: val.pixel_width,
            pixel_height: val.pixel_height,
        }
    }
}

pub struct PtySession {
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl PtySession {
    pub fn spawn(
        shell: Option<&str>,
        size: PtySize,
    ) -> Result<(Self, std::sync::mpsc::Receiver<Vec<u8>>), anyhow::Error> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(size.into())
            .map_err(|e| anyhow::anyhow!("failed to open pty: {e}"))?;

        let shell_path = shell
            .map(|s| s.to_string())
            .or_else(|| std::env::var("SHELL").ok())
            .unwrap_or_else(|| "/bin/sh".to_string());

        let mut cmd = CommandBuilder::new(&shell_path);
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| anyhow::anyhow!("failed to spawn shell command '{shell_path}': {e}"))?;

        // Drop the slave in parent process so EOF works correctly when child exits
        drop(pair.slave);

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| anyhow::anyhow!("failed to clone pty reader: {e}"))?;

        let writer = pair
            .master
            .take_writer()
            .map_err(|e| anyhow::anyhow!("failed to take pty writer: {e}"))?;

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<u8>>();

        // Spawn reader thread
        thread::Builder::new()
            .name("arclet-pty-reader".to_string())
            .spawn(move || {
                let mut reader = reader;
                let mut buf = [0u8; 8192];
                loop {
                    match reader.read(&mut buf) {
                        Ok(0) => {
                            // EOF
                            break;
                        }
                        Ok(n) => {
                            if sender.send(buf[..n].to_vec()).is_err() {
                                break;
                            }
                        }
                        Err(err) => {
                            if err.kind() != std::io::ErrorKind::Interrupted {
                                break;
                            }
                        }
                    }
                }
            })
            .map_err(|e| anyhow::anyhow!("failed to spawn pty reader thread: {e}"))?;

        let session = Self {
            master: Arc::new(Mutex::new(pair.master)),
            child: Arc::new(Mutex::new(child)),
            writer: Arc::new(Mutex::new(writer)),
        };

        Ok((session, receiver))
    }

    pub fn write_all(&self, data: &[u8]) -> std::io::Result<()> {
        if let Ok(mut writer) = self.writer.lock() {
            writer.write_all(data)?;
            writer.flush()?;
        }
        Ok(())
    }

    pub fn resize(&self, size: PtySize) -> Result<(), anyhow::Error> {
        if let Ok(master) = self.master.lock() {
            master
                .resize(size.into())
                .map_err(|e| anyhow::anyhow!("failed to resize pty: {e}"))?;
        }
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        if let Ok(mut child) = self.child.lock() {
            match child.try_wait() {
                Ok(None) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn kill(&self) -> Result<(), anyhow::Error> {
        if let Ok(mut child) = self.child.lock() {
            child
                .kill()
                .map_err(|e| anyhow::anyhow!("failed to kill child: {e}"))?;
        }
        Ok(())
    }
}
