# Arclet Terminal

## Overview

Arclet Terminal is a Rust-based GTK4 terminal emulator project. The long-term goal is to build a modern terminal emulator with Kitty-level capabilities while using native Linux desktop integration through GTK4.

The project currently has two backend paths:

1. **VTE backend** — default, working, and useful as a stable reference implementation.
2. **Ghostty/libghostty-vt backend** — prototype backend intended to become the product's real engine once native dependency and rendering work is mature.

## Goals

1. Provide a usable GTK4 desktop terminal application.
2. Build toward a custom terminal engine rather than remaining a VTE-only wrapper.
3. Support modern terminal behavior comparable to Kitty, including images and advanced protocol support.
4. Use Ghostty/libghostty-vt for terminal parsing and state where practical.
5. Implement a high-performance renderer suitable for text, styled cells, and graphics placements.
6. Keep the project modular so PTY, parser/state, renderer, input, and UI shell can evolve independently.

## Core User Flow

1. User launches Arclet Terminal.
2. App opens a native GTK4 window.
3. App starts the user's configured shell from `$SHELL`, falling back to `/bin/sh`.
4. User interacts with the shell through keyboard input.
5. Terminal output is parsed into terminal state.
6. Terminal state is rendered into the GTK window.
7. Future image/graphics output is decoded, cached, placed, and rendered in the terminal grid.

## Features

### Current Features

- GTK4 application window.
- Default VTE terminal widget backend.
- Shell spawning through VTE in the default backend.
- Optional Ghostty/libghostty-vt prototype backend.
- PTY-backed shell in the Ghostty backend using `portable-pty`.
- Basic Cairo text rendering in the Ghostty backend.
- Basic keyboard input mapping in the Ghostty backend.

### Target Features

- Robust PTY lifecycle management.
- Correct resize propagation to the PTY child process.
- Full keyboard handling, including Kitty keyboard protocol support.
- Mouse reporting modes.
- Scrollback, search, copy, paste, and selection.
- Styled text rendering: foreground/background colors, bold, italic, underline, strikethrough, cursor styles, hyperlinks.
- Image support, with priority on Kitty graphics protocol compatibility.
- Texture caching and graphics placement model.
- High-performance renderer, likely GTK4 `GLArea`, OpenGL, Vulkan, or `wgpu` integration.
- Tabs and panes/splits after core rendering and protocol behavior are stable.

## Scope

### In Scope

- Rust desktop application development.
- GTK4 native Linux UI.
- Terminal parser/state integration through Ghostty/libghostty-vt or another modern terminal core if justified.
- PTY management and shell execution.
- Renderer architecture for text and images.
- Kitty-compatible image/graphics support.
- Documentation and context updates as architecture evolves.

### Out Of Scope For Now

- Windows and macOS support.
- Remote terminal protocol/SSH client features beyond running shell commands locally.
- AI assistant features inside the terminal.
- Cloud sync, account systems, or collaboration.
- Plugin system.
- Full settings UI before the core engine is viable.

## Success Criteria

1. Default build compiles and launches a usable GTK4 terminal.
2. Ghostty/custom backend can parse PTY output and render a real shell session.
3. PTY resize, input, and lifecycle behavior are correct.
4. Renderer can display styled text with acceptable performance.
5. Renderer can display Kitty graphics protocol images in grid-correct positions.
6. Architecture remains modular enough to improve backend, renderer, and UI independently.
7. Project documentation accurately reflects which backend is production-ready versus prototype.
