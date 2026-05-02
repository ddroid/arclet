# Progress Tracker

Update this file whenever the current phase, active feature, or implementation state changes.

## Current Phase

- Foundation/scaffolding complete.
- Architecture direction being documented.

## Current Goal

- Keep the default GTK4 terminal usable through VTE while evolving the Ghostty/custom backend toward the real Arclet Terminal engine.
- Define context rules for future implementation toward Kitty-class terminal features, especially image support.

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

## In Progress

- Clarifying long-term backend direction: VTE as fallback/reference, Ghostty/custom backend as intended product engine.
- Planning architecture for Kitty-compatible graphics/image support.

## Next Up

1. Update README to accurately describe VTE default versus Ghostty prototype status.
2. Refactor source layout into clearer modules: app, backend, pty, input, render, graphics.
3. Define a backend abstraction so VTE and Ghostty paths can coexist cleanly.
4. Fix Ghostty backend build blockers when `deps.files.ghostty.org` downloads are reachable or a vendoring/cache strategy is chosen.
5. Implement PTY resize propagation in the Ghostty backend.
6. Replace prototype Cairo rendering path with a render-scene extraction layer.
7. Research Kitty graphics protocol implementation requirements and how they map to Ghostty/libghostty-vt.

## Open Questions

- Does `libghostty-vt` expose enough graphics protocol state for Kitty image support, or do we need a separate image protocol parser/placement layer?
- Should the future renderer use GTK4 `GLArea`, OpenGL directly, Vulkan, or `wgpu` embedded in GTK?
- Should Arclet implement only Kitty graphics protocol first, or also sixel/iTerm2 inline images?
- What minimum Linux/GTK runtime versions should be supported?
- Should VTE remain available as a permanent fallback backend or only as a temporary development scaffold?

## Architecture Decisions

- Use Rust and GTK4 for the application shell.
- Keep VTE as the default working backend for now.
- Treat VTE as a fallback/reference, not the final engine for Kitty-level features.
- Treat Ghostty/libghostty-vt as the preferred long-term terminal core direction while it remains technically viable.
- Keep PTY I/O off the GTK main thread.
- Keep terminal state owned on the GTK main thread for the Ghostty backend.
- Do not make Cairo row text rendering the long-term renderer architecture.
- Image support must be designed around protocol parsing, placement state, decode/cache, and renderer composition.

## Session Notes

- The Ghostty backend build previously failed because `libghostty-vt-sys` could not download dependencies from `deps.files.ghostty.org` within the available network/time window.
- Default `cargo check` succeeds using VTE.
- The parent `context/` folder belonged to a web app; these context files intentionally replace those rules with desktop Rust/terminal-specific rules.
