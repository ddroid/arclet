# Arclet Terminal

Arclet Terminal is a Rust GTK4 terminal emulator prototype powered by [`libghostty-vt`](https://crates.io/crates/libghostty-vt), Ghostty's VT parsing and terminal state library.

## Requirements

- Rust 1.94+
- GTK4 development libraries
- Zig 0.15.2 for building the vendored Ghostty native library used by `libghostty-vt-sys`

This workspace includes a local Zig install at `.tools/zig` when bootstrapped by Jcode. It is ignored by git.

## Build

```bash
PATH="$PWD/.tools/zig:$PATH" cargo check
PATH="$PWD/.tools/zig:$PATH" cargo run
```

## Current status

- GTK4 application window
- PTY-backed shell
- VT parsing/state via `libghostty-vt`
- Basic Cairo text rendering and keyboard input

This is intentionally a small foundation. The next major improvements are proper styled rendering, PTY resize propagation, mouse support, and using libghostty-vt's render-state APIs for performance.
