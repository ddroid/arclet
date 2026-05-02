# Code Standards

## General

- Keep modules small and single-purpose.
- Fix root causes instead of layering workarounds.
- Respect the boundaries defined in `architecture-context.md`.
- Prefer explicit backend-specific modules over feature-flag spaghetti in one file.
- Keep the default build compiling after every meaningful change.

## Rust

- Use idiomatic Rust 2024.
- Prefer clear ownership and borrowing over shared mutable state.
- Use `Result` for fallible initialization and I/O paths.
- Avoid `unwrap()`/`expect()` outside app startup or tests unless failure is genuinely unrecoverable.
- Do not hide important errors with silent `_ =` unless the operation is best-effort and documented by context.
- Keep public APIs narrow and explicit.
- Run `cargo fmt` before committing.
- Run `cargo check` for the relevant feature set before claiming completion.

## GTK4 / GLib

- GTK widgets and GTK state must be used from the main thread.
- Use GTK event controllers for keyboard and mouse input.
- Do not block the GLib main loop with PTY reads, image decoding, network, or native builds.
- Use GLib channels, standard channels polled by timers, or other main-thread-safe handoff patterns for background work.
- Keep window/application setup separate from terminal backend implementation.

## PTY and Process Handling

- PTY read loops belong on background threads or async tasks, not the GTK main thread.
- PTY output must be treated as untrusted bytes.
- Propagate terminal resize to both terminal state and PTY child process once resize support is implemented.
- Shell selection should respect `$SHELL`, with `/bin/sh` as fallback.
- Environment variables like `TERM` must be selected intentionally and documented.

## Terminal Core

- Prefer `libghostty-vt` for custom backend parsing/state while it remains viable.
- Do not mutate `libghostty-vt::Terminal` off the GTK main thread.
- Register effects callbacks for terminal responses that must be written back to the PTY.
- Use Ghostty render-state APIs for performance once basic correctness is established.
- Avoid manually interpreting escape sequences outside the terminal core unless implementing a protocol not covered by the core.

## Rendering

- Current Cairo rendering is prototype-only.
- Do not build long-term graphics/image architecture around per-row `show_text` calls.
- Separate scene extraction from drawing.
- Renderer must eventually support styled cells, cursor, selection, underlines, hyperlinks, and images.
- Image support must use a placement/texture model, not direct ad-hoc draws inside terminal parsing logic.
- Measure performance when changing render loops.

## Feature Flags

- `vte` is the default working backend.
- `ghostty` is the prototype custom backend.
- Avoid enabling both backends for the same runtime path unless explicitly designing a backend switcher.
- Do not add dependencies to default features unless required for the default app.

## File Organization

- `src/main.rs` — backend selection and minimal app entry dispatch.
- `src/ghostty_backend.rs` — temporary location for the Ghostty prototype backend.
- Future preferred structure:
  - `src/app/` — GTK app/window shell.
  - `src/backend/` — backend trait and backend implementations.
  - `src/pty/` — PTY lifecycle and process management.
  - `src/input/` — keyboard/mouse encoding.
  - `src/render/` — scene extraction and renderer implementations.
  - `src/graphics/` — image protocol, decode, placement, and texture cache.

## Documentation

- Update context files when backend strategy, renderer strategy, or product scope changes.
- Update `progress-tracker.md` after meaningful implementation changes.
- README must not overstate Ghostty support until that backend builds and runs reliably.
