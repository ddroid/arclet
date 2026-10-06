# Power-User Terminal Feature Matrix

## Purpose

This document turns the vague target of "Kitty-class" behavior into a concrete feature list for Arclet Terminal.

Arclet's product direction remains:

- Native Linux desktop shell through GTK4.
- VTE only as a temporary scaffold until the custom backend path is viable.
- Custom terminal core path using Ghostty/libghostty-vt or an equivalent modern terminal core.
- Arclet-owned PTY, input, renderer, and image/graphics architecture.

## Research Sources

- Kitty overview: https://sw.kovidgoyal.net/kitty/overview/
- Kitty graphics protocol: https://sw.kovidgoyal.net/kitty/graphics-protocol/
- Kitty keyboard protocol: https://sw.kovidgoyal.net/kitty/keyboard-protocol/
- Ghostty VT reference: https://ghostty.org/docs/vt/reference
- Ghostty repository roadmap/status: https://github.com/ghostty-org/ghostty
- Ghostling minimal libghostty terminal: https://github.com/ghostty-org/ghostling

## Feature Priority Levels

- **Foundation** — required before the custom backend can replace the VTE scaffold.
- **Power-user V1** — expected by serious daily terminal users.
- **Kitty-class V1** — required for the project's differentiating image/protocol goal.
- **Later** — valuable, but should not block the first custom-backend alpha.

## Compatibility Matrix

| Area | Feature | Priority | Notes |
| --- | --- | --- | --- |
| PTY/process | Spawn `$SHELL` with `/bin/sh` fallback | Foundation | Already present in both paths, but custom path needs lifecycle hardening. |
| PTY/process | Correct PTY resize propagation | Foundation | Must resize both terminal state and kernel PTY so child receives `SIGWINCH`. |
| PTY/process | Clean process shutdown | Foundation | Closing the window must not leave orphan shells. |
| PTY/process | EOF/read-error handling | Foundation | Reader thread must exit cleanly and notify UI. |
| PTY/process | Intentional `TERM` value | Foundation | Do not claim `xterm-ghostty` compatibility until behavior is proven. |
| VT emulation | ECMA-48/xterm-compatible control handling | Foundation | Strong reason to prefer libghostty-vt or another proven terminal core. |
| VT emulation | Alternate screen | Foundation | Required for vim, less, htop, tmux-like tools. |
| VT emulation | Scroll regions and origin mode | Foundation | Required for real TUI correctness. |
| VT emulation | 16-color, 256-color, truecolor | Foundation | Required for modern shell/TUI use. |
| VT emulation | SGR styles | Foundation | Bold, italic, dim, inverse, underline, strikethrough. |
| VT emulation | Cursor visibility and cursor styles | Foundation | Must render block/bar/underline and blink policy later. |
| VT emulation | OSC title handling | Power-user V1 | Window/tab title integration. |
| VT emulation | OSC 8 hyperlinks | Power-user V1 | Needs scene support and click handling. |
| VT emulation | OSC 52 clipboard | Power-user V1 | Useful over SSH; must be permission-controlled. |
| Rendering | Renderer-independent scene model | Foundation | Extract cells, styles, cursor, selections, images before drawing. |
| Rendering | Stable font metrics | Foundation | Grid correctness depends on this. |
| Rendering | Styled cell backgrounds | Foundation | Required for TUI apps. |
| Rendering | Dirty-region tracking | Foundation | Needed for performance and should use libghostty-vt render-state dirty data where viable. |
| Rendering | Wide character and grapheme handling | Foundation | Emoji/CJK/powerline correctness. |
| Rendering | Font fallback | Power-user V1 | Required for multilingual text and symbols. |
| Rendering | Ligatures | Later | Kitty supports ligatures; terminal users are split. Must be configurable. |
| Rendering | GPU texture cache for glyphs | Power-user V1 | Needed for high-throughput output. |
| Rendering | GPU texture cache for images | Kitty-class V1 | Required for Kitty graphics performance. |
| Input | Basic printable text input | Foundation | Already partially present in custom path. |
| Input | Function/navigation keys | Foundation | Current custom mapping is too small. |
| Input | Modifiers for arrows/function keys | Foundation | Required by shells, editors, tmux. |
| Input | Bracketed paste | Foundation | Safety and shell/editor correctness. |
| Input | IME/preedit plan | Power-user V1 | Needed for non-English input. |
| Input | Focus reporting | Power-user V1 | Supported by terminal apps when enabled. |
| Input | Kitty keyboard protocol | Kitty-class V1 | Kitty protocol supports unambiguous modifiers, repeat/release, alternate keys, associated text. |
| Mouse | Selection by drag | Foundation | Required daily UX. |
| Mouse | Double/triple-click selection | Power-user V1 | Kitty supports word/line selection patterns. |
| Mouse | Column/block selection | Power-user V1 | Useful for logs/tables. |
| Mouse | Shift override when app captures mouse | Power-user V1 | Kitty supports selecting even when terminal app has mouse. |
| Mouse | X10/normal/button/any-event tracking | Power-user V1 | Required by TUI apps. |
| Mouse | SGR mouse reporting | Power-user V1 | Modern default. |
| Scrollback | Scroll wheel and keyboard scrollback | Foundation | Must be custom, not VTE-owned. |
| Scrollback | Search | Power-user V1 | Needed for real daily usage. |
| Scrollback | Scrollback pager/export | Later | Kitty can open scrollback in pager while preserving formatting. |
| Clipboard | System clipboard copy/paste | Foundation | GTK clipboard integration. |
| Clipboard | Primary selection | Power-user V1 | Linux-native terminal behavior. |
| Clipboard | Multiple internal buffers | Later | Kitty supports named buffers; not needed early. |
| Graphics | Kitty graphics protocol parser/integration | Kitty-class V1 | Core differentiator. |
| Graphics | Image transmission: inline/direct data | Kitty-class V1 | Start with PNG and direct payload first. |
| Graphics | Image IDs and placement IDs | Kitty-class V1 | Required by Kitty protocol. |
| Graphics | Grid-relative placements | Kitty-class V1 | Images must anchor to cells and scroll/redraw correctly. |
| Graphics | Source rectangles and scaling by rows/cols | Kitty-class V1 | Required for compatibility. |
| Graphics | Z-index relative to text/backgrounds | Kitty-class V1 | Negative z-index allows text over images. |
| Graphics | Deletion commands and lifecycle | Kitty-class V1 | Required to avoid stale images and memory leaks. |
| Graphics | Quotas/eviction | Kitty-class V1 | Kitty recommends quotas to avoid denial-of-service behavior. |
| Graphics | Unicode placeholders | Later | Important for vim/tmux integration, but can follow base placements. |
| Graphics | Animation | Later | Defer until static images work. |
| Shell integration | Prompt markers/jump-to-prompt | Later | Kitty shell integration supports prompt navigation and command-output actions. |
| Shell integration | Open command output in pager | Later | Useful but not core terminal correctness. |
| Windowing | Single terminal surface | Foundation | Current scope. |
| Windowing | Tabs | Later | Do after custom backend/rendering is stable. |
| Windowing | Splits/panes | Later | Do after session abstraction exists. |
| Windowing | Layout management | Later | Kitty has many layouts; Arclet can start simpler. |
| Config | Human-editable config | Power-user V1 | Do after core engine viability. |
| Config | Themes | Power-user V1 | Centralize colors before full theming. |
| Config | Keybindings | Power-user V1 | Required once app shortcuts exist. |
| Remote/control | Remote control API | Later | Kitty has powerful remote control; defer until local model is stable. |
| Extensibility | Kittens/plugin equivalent | Later | Out of scope for now. |
| Packaging | `.desktop`, AppStream, icon | Later | Needed before release, not before engine viability. |

## Alpha Definition For The Custom Backend

The VTE scaffold can stop being the default only after the custom backend can pass this minimum alpha set:

1. Launch shell from `$SHELL`.
2. Render prompt and command output with correct basic styles.
3. Resize correctly: terminal state and PTY child both update.
4. Run `vim`, `less`, `htop`, and `tmux` without obvious input/render breakage.
5. Support alternate screen, truecolor, cursor, and common SGR styles.
6. Support scrollback and basic mouse selection.
7. Support bracketed paste.
8. Shut down without orphaning child processes.
9. Keep the GTK main loop responsive under large output.

## Recommended Compatibility Test Matrix

| Test | Command or Scenario | Expected Result |
| --- | --- | --- |
| Shell | open terminal | Prompt appears and input echoes. |
| Resize | `stty size`; resize window; `stty size` again | Rows/cols change and TUI apps receive `SIGWINCH`. |
| Truecolor | truecolor test script | Smooth 24-bit color gradients. |
| SGR | ANSI style/color demo | Bold, italic, underline, inverse, colors render correctly. |
| Alternate screen | `vim`, `less`, `htop` | Screen enters/exits cleanly. |
| Mouse | app enabling SGR mouse mode | Mouse events arrive only when requested. |
| Paste | paste multiline text into shell/editor | Bracketed paste used when enabled. |
| Scrollback | produce thousands of lines | Scrollback remains usable and bounded. |
| OSC title | shell changes title | Window/tab title updates when implemented. |
| Hyperlink | OSC 8 demo | Link hit-testing works when implemented. |
| Kitty keyboard | app opts in with `CSI > 1 u` | Modified keys encode unambiguously. |
| Kitty image | `kitten icat`, `timg`, or compatible tool | Image appears at correct cell placement. |

## Recommendation

Use this feature matrix as the replacement for the phrase "Kitty-class" in planning. A feature should not enter implementation until it has:

- an owner subsystem,
- a protocol/source reference,
- a test or smoke scenario,
- renderer implications,
- PTY/input implications where relevant,
- and a priority level from this matrix.
