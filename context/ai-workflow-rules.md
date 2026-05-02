# Development Workflow

## Approach

Build Arclet Terminal incrementally using a spec-driven workflow. Context files define what to build, how to build it, and what the current implementation state is. Always implement against these specs rather than inventing behavior from scratch.

## Scoping Rules

- Work on one feature unit or subsystem at a time.
- Prefer small, verifiable increments over broad speculative rewrites.
- Keep PTY, terminal parser/state, renderer, input, and GTK window shell boundaries clear.
- Do not combine unrelated backend, renderer, and UI changes unless the task explicitly requires it.

## When To Split Work

Split an implementation step if it combines:

- PTY lifecycle changes and renderer changes.
- Parser/state changes and settings/config changes.
- Image protocol parsing and GPU renderer implementation.
- Keyboard input protocol work and mouse reporting work.
- VTE backend changes and Ghostty backend changes without a shared abstraction.
- Behavior that is not clearly defined in the context files.

If a change cannot be verified quickly with `cargo check`, a smoke run, or a targeted test, the scope is probably too broad.

## Handling Missing Requirements

- Do not invent terminal behavior that is not defined in context files or protocol documentation.
- If behavior is ambiguous, add an open question to `progress-tracker.md` before implementing.
- For terminal protocols, prefer reading official protocol documentation or upstream source before coding assumptions.
- If Ghostty/libghostty-vt support for a target capability is unclear, document the uncertainty before building around it.

## Validation Requirements

Before claiming a change is done:

1. Run `cargo fmt` if Rust files changed.
2. Run `cargo check` for the default feature set.
3. If Ghostty backend files changed, attempt `PATH="$PWD/.tools/zig:$PATH" cargo check --no-default-features --features ghostty` when dependency downloads are available.
4. For UI/runtime changes, run or describe a smoke test.
5. Update `context/progress-tracker.md` with actual state and limitations.

## Protected Foundation Areas

Do not casually edit third-party library internals, vendored native sources, or generated build output.

This includes:

- Cargo registry sources.
- `target/` build output.
- Vendored Ghostty source under build directories.
- Local Zig toolchain under `.tools/zig`.

If a dependency patch is needed, make it explicit through Cargo patching or a documented fork strategy.

## Keeping Docs In Sync

Update the relevant context file whenever implementation changes:

- Backend strategy.
- Renderer strategy.
- PTY/input architecture.
- Image/graphics support plan.
- Feature scope.
- Build requirements.
- Coding standards.

Progress state must reflect the actual state of the implementation, not the intended future state.

## Before Moving To The Next Unit

1. The current unit works end to end within its defined scope.
2. No invariant defined in `architecture-context.md` was violated.
3. The default build still compiles.
4. `progress-tracker.md` reflects completed work, limitations, and next steps.
