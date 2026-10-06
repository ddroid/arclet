# Arclet Terminal

Arclet Terminal is a Rust GTK4 terminal emulator project building toward a custom, Kitty-class terminal engine with native Linux desktop integration.

The current default build uses VTE as a temporary working scaffold so the app remains usable while the custom backend matures. VTE is not the intended final engine. The long-term direction is a custom GTK4 terminal emulator backed by Ghostty/libghostty-vt or an equivalent modern terminal core, with Arclet-owned PTY, input, rendering, and graphics architecture.

## Requirements

- Rust 1.94+
- GTK4 development libraries
- VTE GTK4 development libraries for the default scaffold backend
- Zig 0.15.2 for the optional Ghostty/libghostty-vt prototype backend

This workspace includes a local Zig install at `.tools/zig` when bootstrapped by Jcode. It is ignored by git.

## Build

Default scaffold backend:

```bash
cargo check
cargo run
```

Optional Ghostty/custom backend prototype:

```bash
PATH="$PWD/.tools/zig:$PATH" cargo check --no-default-features --features ghostty
PATH="$PWD/.tools/zig:$PATH" cargo run --no-default-features --features ghostty
```

The Ghostty path may require network access for native dependency downloads performed by `libghostty-vt-sys`.

## Current Status

Default backend:

- GTK4 application window.
- Working shell through VTE.
- VTE-provided terminal emulation, rendering, scrollback, selection, clipboard, and resize behavior.

Custom backend prototype:

- Optional Ghostty/libghostty-vt parser/state experiment.
- PTY-backed shell through `portable-pty`.
- Basic Cairo text rendering.
- Basic keyboard input mapping.
- Known incomplete areas: reliable native build path, PTY resize propagation to the child process, robust input encoding, styled rendering, selection, scrollback UX, graphics/image support, and renderer performance.

## Product Direction

Arclet should not remain a thin VTE wrapper. VTE exists to preserve a usable baseline while the custom backend path is proven. The target product is a modular terminal emulator with separate PTY, parser/state, input, renderer, and graphics subsystems.

The immediate focus is to validate the custom backend foundation before advanced features:

1. Stabilize the Ghostty/libghostty-vt build path or choose an equivalent terminal core if Ghostty is not viable.
2. Implement correct PTY lifecycle and resize behavior.
3. Replace direct Cairo row drawing with a renderer-independent scene model.
4. Choose a renderer technology that can support styled text, cursor, selection, scrollback, and future grid-positioned images.
5. Design Kitty-compatible graphics support before implementation.
