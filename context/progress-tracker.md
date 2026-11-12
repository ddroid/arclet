# Progress Tracker

Update this file whenever the current phase, active feature, or implementation state changes.

## Current Phase

- Foundation/scaffolding complete.
- Architecture direction documented.
- Broad completion roadmap defined in `context/specs/index.md`.
- Research reports added for power-user terminal features and backend/renderer direction.

## Current Goal

- Keep the default GTK4 terminal usable through VTE while evolving the Ghostty/custom backend toward the real Arclet Terminal engine.
- Define context rules for future implementation toward Kitty-class terminal features, especially image support.
- Use `context/specs/index.md` as the ordered high-level roadmap for completing the app.
- Treat VTE as a temporary scaffold until the custom backend can replace it.

## Completed

- Created project directory at `/home/ddroid/Jcode/arclet-terminal`.
- Initialized Rust binary crate.
- Added GTK4 Rust bindings.
- Added VTE GTK4 backend as default feature.
- Added optional Ghostty/libghostty-vt backend prototype.
- Added `portable-pty` integration for the Ghostty backend.
- Added basic Cairo text rendering for the Ghostty backend prototype.
- Installed local Zig 0.15.2 under `.tools/zig` for Ghostty native builds.
- Verified default build with `cargo check`.
- Created initial git commit: `b4dab72 Scaffold GTK4 terminal emulator`.
- Created project `context/` folder and documented project rules.
- Created `context/specs/index.md` with the ordered broad task roadmap to reach a complete Kitty-class terminal emulator.
- Updated README to describe VTE as the default temporary scaffold and Ghostty/custom as the product path.
- Added `context/research/power-user-terminal-feature-matrix.md`.
- Added `context/research/backend-renderer-recommendation-report.md`.
- **Phase 3 Source Architecture Refactor Complete:**
  - Modularized subsystems under `src/`:
    - `src/app/`: `MainWindow`, `TabManager` (GTK4 Notebook multi-tab), `TerminalPane` (GTK4 Paned splits).
    - `src/backend/`: `TerminalBackend` trait decoupling UI from emulation engines with factory `create_backend`.
    - `src/backend/vte/`: VTE reference backend implementation.
    - `src/backend/alacritty/`: Native Rust terminal emulation backend (`alacritty_terminal` 0.26) with Cairo rendering, dynamic resize, and Kitty image placement hooks.
    - `src/pty/`: `PtySession` abstraction with `portable-pty` managing lifecycle, thread-safe master resize, and background async reader.
    - `src/input/`: `ShortcutManager` (keybindings map) and `KittyKeyboardEncoder` (Kitty keyboard protocol / VT escape sequences).
    - `src/graphics/`: `KittyParser` (Kitty Graphics Protocol escape parser), `ImageRegistry` (image cache and grid placement tracking), and `KittyGraphicsInterceptor` (in-stream APC extractor).
    - `src/config/`: `TerminalConfig`, `ColorScheme` (Arclet Dark theme palette), and `KeybindingConfig`.
- Configured keybindings: `Ctrl+Shift+T` (new tab), `Ctrl+Shift+W` (close tab), `Ctrl+Shift+D` (split horizontal), `Ctrl+Shift+E` (split vertical), `Ctrl+PageDown`/`Ctrl+PageUp` (tab navigation), `Ctrl+Shift+C`/`Ctrl+Shift+V` (copy/paste), `Ctrl++`/`Ctrl+-`/`Ctrl+0` (zoom controls).
- Added comprehensive unit tests in `tests/subsystems_test.rs` passing cleanly.

## In Progress

- GPU acceleration renderer prototyping (`GLArea` + OpenGL) for high-performance glyph atlas and Kitty graphics texture rendering.
- Kitty graphics pixel decoding (PNG/RGBA) into GPU textures.

## Next Up

1. Replace prototype Cairo text renderer in the custom backend with a GTK Snapshot / Pango or OpenGL `GLArea` renderer.
2. Complete Kitty graphics texture upload and rendering onto the terminal canvas.
3. Add configuration file loading (`~/.config/arclet/config.toml`).
4. Session persistence and search bar overlay.

## Open Questions

- Does `libghostty-vt` expose enough image placement/pixel data for Kitty graphics, or does Arclet need an owned graphics parser/placement layer?
- Should the future renderer use GTK4 `GLArea`, OpenGL directly, Vulkan, or `wgpu` embedded in GTK?
- Should Arclet implement only Kitty graphics protocol first, or also sixel/iTerm2 inline images?
- What minimum Linux/GTK runtime versions should be supported?
- If Ghostty/libghostty-vt remains blocked or insufficient, which equivalent modern terminal core should replace it?

## Architecture Decisions

- Use Rust and GTK4 for the application shell.
- Keep VTE as the default working backend for now.
- Treat VTE as a temporary scaffold/reference, not a permanent fallback or final engine for Kitty-level features.
- Treat Ghostty/libghostty-vt as the preferred long-term terminal core direction while it remains technically viable.
- Keep PTY I/O off the GTK main thread.
- Keep terminal state owned on the GTK main thread for the Ghostty backend.
- Do not make Cairo row text rendering the long-term renderer architecture.
- Image support must be designed around protocol parsing, placement state, decode/cache, and renderer composition.
- Use the research reports in `context/research/` as current guidance for power-user feature scope, backend viability, renderer choice, text shaping, image support, and PTY resize strategy.

## Session Notes

- The Ghostty backend build previously failed because `libghostty-vt-sys` could not download dependencies from `deps.files.ghostty.org` within the available network/time window.
- Default `cargo check` succeeds using VTE.
- The parent `context/` folder belonged to a web app; these context files intentionally replace those rules with desktop Rust/terminal-specific rules.
- `context/specs/index.md` is now the ordered broad roadmap. Use it to choose and sequence future implementation specs.
- VTE is now explicitly chosen as a temporary scaffold until the custom Ghostty/backend path can replace it.
- Recommended renderer path from research: GTK Snapshot/Pango correctness renderer first, then GTK4 `GLArea` + OpenGL with `cosmic-text`/`swash` glyph atlas prototype.
