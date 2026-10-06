# Arclet Terminal Agent Context

## Application Building Context

Read the following files in order before implementing or making any architectural decision:

1. `context/project-overview.md` — product definition, goals, features, and scope
2. `context/architecture-context.md` — system structure, backend strategy, rendering model, PTY model, and invariants
3. `context/ui-context.md` — terminal visual direction, theme, typography, renderer expectations, and window conventions
4. `context/code-standards.md` — Rust, GTK, terminal-core, rendering, and platform implementation rules
5. `context/ai-workflow-rules.md` — development workflow, scoping rules, validation expectations, and delivery approach
6. `context/progress-tracker.md` — current phase, completed work, open questions, and next steps

Update `context/progress-tracker.md` after each meaningful implementation change.

If implementation changes the architecture, backend strategy, feature scope, rendering model, or standards documented in the context files, update the relevant context file before continuing.

## Product Direction

Arclet Terminal is not intended to remain a thin VTE wrapper. VTE is allowed as a temporary working scaffold/reference backend while the custom backend is not yet viable, but the long-term product direction is a custom GTK4 terminal emulator powered by Ghostty/libghostty-vt or equivalent modern terminal-core components.

The target capability level is Kitty-class terminal functionality, including modern keyboard handling, images, graphics protocols, scrollback, tabs/splits eventually, rich styling, and high-performance rendering.

## Critical Agent Rules

- Do not silently replace the Ghostty/custom backend direction with VTE-only work.
- Do not design long-term architecture around VTE as a permanent fallback unless that decision is explicitly revisited.
- Do not treat the current Cairo renderer as final architecture.
- Do not add terminal features without considering protocol correctness, renderer implications, and PTY behavior.
- Prefer verifiable increments: compile, run smoke tests, and document limitations.
- Keep the default build usable even while the Ghostty backend is blocked by external native dependency downloads.
