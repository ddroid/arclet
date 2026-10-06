# Architecture Context

## Stack

| Layer | Technology | Role |
| --- | --- | --- |
| Language | Rust 2024 | Application, terminal engine integration, PTY, rendering glue |
| Desktop UI | GTK4 Rust bindings | Native Linux app lifecycle, windows, widgets, input controllers |
| Event loop | GLib | GTK main loop, timers, async callbacks |
| Default terminal backend | VTE GTK4 | Temporary working scaffold/reference terminal widget |
| Target terminal core | libghostty-vt | VT parsing, terminal state, scrollback, modes, effects callbacks |
| PTY | portable-pty | Shell spawning and PTY I/O for custom backend |
| Prototype renderer | GTK4 DrawingArea + Cairo | Basic text rendering for Ghostty backend prototype |
| Future renderer | GTK4 GLArea/OpenGL/Vulkan/wgpu | High-performance text and image rendering |
| Native build helper | Zig 0.15.2 | Required by `libghostty-vt-sys` when building vendored Ghostty native library |

## Current System Boundaries

- `src/main.rs` — application entry point and feature-selected backend dispatch.
- `src/ghostty_backend.rs` — optional custom backend prototype using `portable-pty`, `libghostty-vt`, GTK4 DrawingArea, and Cairo.
- `context/` — project rules, architecture, workflow, standards, UI direction, and progress state.
- `Cargo.toml` — feature flags and backend dependency selection.
- `.tools/zig` — local ignored Zig toolchain used for Ghostty native builds when available.

## Backend Strategy

### VTE Backend

The VTE backend is the default because it currently compiles and provides a functional terminal quickly.

Responsibilities handled by VTE:

- PTY allocation and shell spawning.
- Terminal parsing and emulation.
- Text rendering.
- Selection, clipboard, scrollback, resize behavior.

VTE is a temporary scaffold/reference implementation, not the final product engine for Kitty-class features. Do not design long-term architecture around preserving VTE as a permanent supported backend unless that decision is explicitly revisited.

### Ghostty/libghostty-vt Backend

The Ghostty backend is the intended long-term direction.

Responsibilities:

- `portable-pty` owns shell process and byte I/O.
- Reader thread moves PTY output to the GTK main thread through a channel.
- `libghostty-vt::Terminal` parses bytes and owns terminal state.
- GTK input controllers map user input to bytes written to the PTY.
- Renderer reads terminal state and draws visible cells.

Current renderer uses `grid_ref(Point::Viewport(...))` and `graphemes()` for simple text drawing. This is acceptable for scaffolding only. Move to render-state APIs and a GPU-aware renderer for real performance.

## Data Flow

```text
User keyboard/mouse
    ↓
GTK input controller
    ↓
Input encoder / protocol mapper
    ↓
PTY writer
    ↓
Shell process
    ↓
PTY reader thread
    ↓
Main-thread channel drain
    ↓
libghostty-vt Terminal::vt_write
    ↓
Terminal state / effects callbacks
    ↓
Renderer scene extraction
    ↓
GTK drawing surface
```

## Image/Graphics Model Target

Kitty-compatible image support requires explicit architecture, not ad-hoc drawing.

Expected components:

- Graphics protocol parser or integration point.
- Image decode pipeline.
- Image registry keyed by protocol image IDs.
- Placement registry mapped to terminal grid coordinates.
- Texture cache with lifecycle and eviction.
- Renderer support for z-ordering text, cell backgrounds, cursor, selections, and image placements.

Do not implement image support as direct one-off Cairo draws inside the text loop.

## Storage Model

Arclet Terminal is currently local-only. There is no database, server, or cloud storage layer.

Persistent state may eventually include:

- Configuration file.
- Theme file.
- Keybinding file.
- Session restoration metadata.

Until a config format is defined, avoid adding persistent state casually.

## Build Model

- Default build: `cargo check` / `cargo run` uses the VTE backend.
- Ghostty build: `PATH="$PWD/.tools/zig:$PATH" cargo run --no-default-features --features ghostty`.
- `libghostty-vt-sys` may need network access to fetch Ghostty dependency tarballs.

## Invariants

1. Default build must remain usable unless a task explicitly targets backend replacement.
2. VTE is a temporary scaffold, not the final architecture for Kitty-level feature parity.
3. Terminal parsing/state, PTY I/O, rendering, and GTK shell should remain separate concerns.
4. All GTK widget access must happen on the main thread.
5. `libghostty-vt` terminal objects are not `Send`/`Sync`; keep them owned by the GTK main thread.
6. PTY reader threads may send bytes through channels, but must not mutate GTK or terminal state directly.
7. Do not block the GTK main loop with long reads, native builds, image decoding, or expensive rendering work.
8. Renderer changes must account for text, styles, cursor, selection, scrollback, and images as one composed scene.
