# Arclet Terminal Completion Roadmap

This document is the broad, ordered task index for taking Arclet Terminal from the current scaffold to a fully complete modern terminal emulator with Kitty-class capabilities.

The order matters. Later phases depend on earlier architecture, correctness, and renderer decisions. Do not jump directly to advanced features like images, tabs, or GPU effects until the required foundation is in place.

## Target End State

Arclet Terminal should become a native GTK4 Rust terminal emulator with:

- A reliable local shell/PTY experience.
- A custom terminal engine path, preferably powered by Ghostty/libghostty-vt where viable.
- Kitty-level terminal features, especially graphics/image support.
- High-performance rendering for text, styled cells, cursor, selection, and images.
- Modern input handling: keyboard protocols, mouse modes, paste, IME/preedit.
- Production-quality UX: themes, keybindings, tabs/splits, search, settings, packaging, and tests.

## Phase 0 — Preserve The Working Baseline

**Goal:** Keep the app buildable and runnable at every step.

Tasks:

1. Keep the default VTE backend compiling with `cargo check`.
2. Keep `cargo fmt` clean after Rust changes.
3. Maintain git commits for meaningful milestones.
4. Keep `context/progress-tracker.md` updated after each implementation unit.
5. Avoid breaking the default user-facing terminal while custom backend work is incomplete.

Completion criteria:

- `cargo check` passes on the default feature set.
- App can launch with a working shell through the VTE backend.

## Phase 1 — Documentation And Product Definition

**Goal:** Make the intended product and engineering direction explicit.

Tasks:

1. Keep `context/` files current.
2. Clarify in `README.md` that VTE is a fallback/reference backend and Ghostty/custom is the product direction.
3. Add developer instructions for default build, Ghostty build, and known Ghostty dependency blockers.
4. Document current limitations honestly.
5. Convert this roadmap into smaller implementation specs as work begins.

Completion criteria:

- README and context files agree on architecture and status.
- New contributors can understand the backend strategy without reading chat history.

## Phase 2 — Build System And Dependency Stabilization

**Goal:** Make native dependencies predictable, especially Ghostty/libghostty-vt.

Tasks:

1. Document required system packages: Rust, GTK4 dev libraries, VTE GTK4 dev libraries, pkg-config, Zig.
2. Decide whether Zig should remain locally downloaded under `.tools/zig` or be managed externally.
3. Re-test `libghostty-vt-sys` builds when `deps.files.ghostty.org` is reachable.
4. Investigate caching/vendoring strategy for Ghostty dependency tarballs.
5. Add a simple `justfile`, `Makefile`, or scripts for common commands:
   - `check`
   - `check-ghostty`
   - `run`
   - `run-ghostty`
   - `fmt`
6. Ensure feature combinations are intentional:
   - default: VTE
   - `--no-default-features --features ghostty`: custom Ghostty backend

Completion criteria:

- Default build is one-command reliable.
- Ghostty build path is documented and reproducible when external deps are available.

## Phase 3 — Source Architecture Refactor

**Goal:** Replace the initial scaffold layout with clean subsystem boundaries.

Tasks:

1. Create a structured source tree:
   - `src/app/` — GTK app, window shell, action setup.
   - `src/backend/` — backend trait and backend implementations.
   - `src/backend/vte/` — VTE reference backend.
   - `src/backend/ghostty/` — Ghostty/custom backend.
   - `src/pty/` — PTY lifecycle, resize, reader/writer management.
   - `src/input/` — keyboard, mouse, paste, IME/preedit encoding.
   - `src/render/` — scene extraction, renderer traits, Cairo/GPU implementations.
   - `src/graphics/` — image protocol, image registry, placements, decode/cache.
   - `src/config/` — future themes, keybindings, settings.
2. Move `src/ghostty_backend.rs` into the new structure.
3. Keep feature gates at module boundaries, not scattered through business logic.
4. Define core data types for viewport size, cell metrics, terminal events, and render invalidation.

Completion criteria:

- The codebase has clear modules for app, backend, PTY, input, render, and graphics.
- Default build still passes.

## Phase 4 — Backend Abstraction

**Goal:** Allow VTE and Ghostty/custom backends to coexist cleanly.

Tasks:

1. Define a backend interface for app-level operations:
   - create widget/surface
   - focus terminal
   - spawn shell
   - resize
   - copy/paste
   - shutdown
2. Keep VTE backend as a reference implementation.
3. Implement Ghostty backend through the same app-level lifecycle where possible.
4. Avoid forcing custom renderer concepts into the VTE backend.
5. Add backend selection through Cargo features first, and possibly runtime selection later.

Completion criteria:

- App shell does not need to know backend internals.
- VTE and Ghostty code paths are separated but consistently launched.

## Phase 5 — Ghostty Backend Build And Runtime Viability

**Goal:** Make the custom backend build and run reliably.

Tasks:

1. Resolve `libghostty-vt-sys` native build/dependency downloads.
2. Confirm `libghostty-vt` version and API stability.
3. Build the Ghostty backend with:
   ```bash
   PATH="$PWD/.tools/zig:$PATH" cargo check --no-default-features --features ghostty
   ```
4. Launch the Ghostty backend and run a real shell.
5. Confirm terminal responses are written back through `on_pty_write`.
6. Add basic runtime smoke notes or tests for:
   - shell prompt appears
   - typed command echoes
   - simple command output renders

Completion criteria:

- Ghostty backend compiles.
- Ghostty backend launches a real interactive shell.
- Basic input/output works end to end.

## Phase 6 — PTY Lifecycle Correctness

**Goal:** Make custom backend shell/process behavior correct and robust.

Tasks:

1. Encapsulate PTY pair, child process, reader thread, and writer handle in a dedicated module.
2. Propagate terminal resize to the PTY child process.
3. Handle shell process exit and terminal close cleanly.
4. Handle read errors and EOF without panics.
5. Avoid blocking GTK main thread.
6. Decide how to restart or close sessions.
7. Add tests or smoke scripts for resize and process exit where possible.

Completion criteria:

- Custom backend correctly resizes shell applications like `stty size`, `vim`, `less`, or `htop`.
- Closing the window does not leave orphaned shell processes.

## Phase 7 — Input System

**Goal:** Implement correct terminal input behavior.

Tasks:

1. Replace hardcoded minimal key mapping with a dedicated input encoder.
2. Support modifier combinations for arrows, function keys, Home/End, PageUp/PageDown, Insert/Delete.
3. Support Kitty keyboard protocol behavior if exposed by or compatible with Ghostty terminal state.
4. Implement bracketed paste support.
5. Implement mouse input encoding for terminal mouse tracking modes.
6. Add support plan for IME/preedit text.
7. Ensure app shortcuts do not break TUI applications.

Completion criteria:

- Common shell/TUI apps receive correct keyboard input.
- Mouse reporting works when enabled by terminal applications.
- Pasted text is safe and protocol-correct.

## Phase 8 — Terminal State And Render Scene Extraction

**Goal:** Build a renderer-independent scene model from terminal state.

Tasks:

1. Stop rendering directly from ad-hoc `grid_ref` loops as the long-term path.
2. Investigate and adopt `libghostty-vt` render-state APIs where suitable.
3. Define an Arclet render scene containing:
   - visible cells
   - text graphemes
   - styles
   - cursor
   - selections
   - scroll offsets
   - image placements
4. Track dirty regions and invalidation.
5. Separate scene extraction from drawing backend.

Completion criteria:

- Renderer receives a structured scene, not raw terminal internals.
- Future Cairo/GPU renderers can consume the same scene model.

## Phase 9 — Text Renderer V1: Correctness First

**Goal:** Build a correct styled text renderer before optimizing heavily.

Tasks:

1. Render foreground and background colors.
2. Render bold, italic, dim, underline, strikethrough, inverse, invisible.
3. Render cursor styles.
4. Render selection overlay.
5. Handle wide characters and combining graphemes correctly.
6. Add font metrics calculation instead of fixed constants.
7. Handle font fallback and emoji strategy.
8. Add scrollback viewport rendering.

Completion criteria:

- ANSI color demos render correctly.
- Common TUI apps are visually usable.
- Cell metrics are stable across resize.

## Phase 10 — Scrollback, Selection, Clipboard, And Search

**Goal:** Implement core terminal UX around the custom backend.

Tasks:

1. Implement scrollback navigation.
2. Add mouse selection.
3. Add keyboard selection if desired.
4. Integrate GTK clipboard for copy/paste.
5. Add search over visible content and scrollback.
6. Add URL/hyperlink detection or OSC 8 hyperlink support.
7. Decide selection behavior for wrapped lines and wide characters.

Completion criteria:

- User can select, copy, paste, scroll, and search reliably.
- Behavior matches common terminal expectations.

## Phase 11 — Graphics/Image Protocol Research And Design

**Goal:** Design image support before implementation.

Tasks:

1. Study Kitty graphics protocol in detail.
2. Determine what `libghostty-vt` already parses or exposes for graphics/images.
3. Decide if Arclet needs its own image protocol parser/placement layer.
4. Define image data flow:
   - escape sequence/protocol data
   - transfer assembly
   - decode
   - image registry
   - placement registry
   - render scene integration
   - texture cache
5. Decide supported formats for first implementation: PNG first, then others as needed.
6. Decide whether to support additional protocols later:
   - sixel
   - iTerm2 inline images
   - Unicode placeholders
7. Write a focused graphics architecture spec before coding.

Completion criteria:

- There is a written image architecture spec.
- Unknowns around Ghostty/libghostty-vt image support are resolved or documented.

## Phase 12 — Image Support V1: Kitty Graphics Protocol

**Goal:** Display Kitty protocol images correctly enough for real use.

Tasks:

1. Implement or integrate Kitty graphics protocol parsing.
2. Support image transmission modes selected for V1.
3. Decode image payloads off the GTK main thread.
4. Store image IDs and placements.
5. Handle placement at grid cell coordinates.
6. Render images in the correct order relative to cell backgrounds and text.
7. Implement image deletion/cleanup protocol commands.
8. Add manual tests with Kitty-compatible image tools.

Completion criteria:

- A Kitty graphics protocol test image displays in the correct terminal location.
- Scrolling and redraw behavior remain stable.
- Image memory is bounded by a cache/lifecycle policy.

## Phase 13 — GPU Renderer

**Goal:** Move from prototype Cairo rendering to a renderer suitable for high-performance text and images.

Tasks:

1. Choose renderer technology:
   - GTK4 `GLArea` + OpenGL
   - Vulkan
   - `wgpu` embedded in GTK
2. Prototype rendering a grid of colored cells and text glyphs.
3. Add glyph atlas or text rendering strategy.
4. Add image texture cache.
5. Compose background, cell fills, images, glyphs, selection, and cursor.
6. Measure redraw latency and frame time.
7. Keep a simpler fallback renderer if useful.

Completion criteria:

- Large terminal outputs render smoothly.
- Images render as GPU textures.
- Renderer architecture can support future animations/cursor blink without heavy CPU redraws.

## Phase 14 — Terminal Compatibility And Protocol Coverage

**Goal:** Improve compatibility with real shell/TUI applications.

Tasks:

1. Test with common apps:
   - bash/zsh/fish
   - vim/neovim
   - less/man
   - tmux
   - htop/btop
   - git interactive commands
   - image-capable CLI tools
2. Validate color modes:
   - 16 color
   - 256 color
   - truecolor
3. Validate alternate screen behavior.
4. Validate mouse tracking modes.
5. Validate bracketed paste.
6. Validate OSC title changes and hyperlinks.
7. Validate terminal identity and `TERM` choice.

Completion criteria:

- Common TUI apps are usable without obvious protocol/input/rendering bugs.
- Compatibility gaps are tracked explicitly.

## Phase 15 — Configuration, Themes, And Keybindings

**Goal:** Make the app customizable without destabilizing the core.

Tasks:

1. Choose config format and location.
2. Add theme schema for colors and font settings.
3. Add keybinding schema.
4. Add terminal behavior options:
   - scrollback size
   - cursor style
   - font size
   - shell command
   - TERM value
5. Add live reload where safe.
6. Add validation and friendly errors.

Completion criteria:

- Users can configure basic appearance and behavior without recompiling.
- Bad config does not crash the app.

## Phase 16 — App UX: Tabs, Splits, Search UI, Settings UI

**Goal:** Build terminal application features around the core engine.

Tasks:

1. Add tab model and UI.
2. Add pane/split model and UI.
3. Add search overlay.
4. Add settings/preferences window.
5. Add command palette or shortcut help if desired.
6. Add session naming/title behavior.
7. Add close confirmations only where necessary.

Completion criteria:

- Multiple terminal sessions can run in one window.
- Search/settings/tabs do not interfere with terminal input behavior.

## Phase 17 — Testing And Quality Infrastructure

**Goal:** Make regressions catchable.

Tasks:

1. Add unit tests for pure modules: input encoding, config parsing, protocol parsing, placement model.
2. Add golden tests for render scene extraction.
3. Add integration smoke tests where practical.
4. Add protocol fixture tests for Kitty image commands.
5. Add CI workflow if repository hosting is configured.
6. Add performance benchmarks for render scene extraction and renderer hot paths.

Completion criteria:

- Important parser/input/scene behavior has automated tests.
- Performance regressions can be measured.

## Phase 18 — Packaging And Desktop Integration

**Goal:** Make Arclet installable and desktop-friendly.

Tasks:

1. Add app icon.
2. Add `.desktop` file.
3. Add AppStream metadata.
4. Choose packaging targets:
   - distro package later
   - Flatpak
   - AppImage
   - manual binary release
5. Define runtime dependencies.
6. Test launch from desktop environment.

Completion criteria:

- App can be installed/launched like a normal Linux desktop application.

## Phase 19 — Performance, Memory, And Stability Hardening

**Goal:** Make the app robust under real-world load.

Tasks:

1. Profile large output throughput.
2. Profile scrollback memory use.
3. Profile image cache memory use.
4. Add limits and eviction policies.
5. Avoid unbounded channel growth from PTY output.
6. Handle rapid resize and high-output commands.
7. Fix leaks, orphan processes, and shutdown races.

Completion criteria:

- App remains responsive under high output.
- Image and scrollback memory are bounded.
- Shutdown is clean and repeatable.

## Phase 20 — Release Readiness

**Goal:** Prepare a user-facing release.

Tasks:

1. Complete user documentation.
2. Complete developer documentation.
3. Audit README claims against implemented features.
4. Create known limitations document.
5. Tag initial alpha release once custom backend is usable.
6. Collect feedback and prioritize remaining compatibility gaps.

Completion criteria:

- Users can install, run, configure, and understand current limitations.
- The release does not claim unsupported Kitty/Ghostty features.

## Suggested Immediate Next Tasks

The next implementation work should happen in this order:

1. Update README to match current architecture and status.
2. Add helper commands/scripts for default and Ghostty builds.
3. Refactor source layout into clear modules.
4. Define backend abstraction.
5. Re-attempt and stabilize Ghostty backend build.
6. Implement PTY resize propagation.
7. Replace direct renderer loops with a render scene extraction layer.
8. Write the dedicated Kitty graphics architecture spec before implementing images.
