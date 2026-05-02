# UI Context

## Theme

Arclet Terminal should use a dark technical visual style by default. Light mode is not a priority until the core terminal engine and renderer are stable.

Target feel:

- Near-black terminal background.
- Clear high-contrast text.
- Subtle GTK-native window integration.
- Minimal chrome around the terminal surface.
- Accent colors used sparingly for focus rings, tabs, search, and status indicators.

## Color Direction

| Role | Suggested Value | Notes |
| --- | --- | --- |
| Terminal background | `#08090d` | Near-black, not pure black |
| Surface background | `#11131a` | For tabs/settings/sidebar later |
| Elevated surface | `#181b24` | Dialogs/popovers later |
| Primary text | `#eef1f8` | Default foreground |
| Secondary text | `#a8afbd` | Muted UI text |
| Border | `#2a2f3a` | Subtle separators |
| Accent cyan | `#00c8d4` | Brand/focus accent |
| Accent purple | `#8b82ff` | Optional secondary accent |
| Error | `#ff5c66` | Errors/destructive states |
| Success | `#34d399` | Success states |
| Warning | `#fbbf24` | Warnings |

These values are directional. When a real theme/config system is introduced, colors should be centralized instead of hardcoded across rendering code.

## Typography

### Terminal Text

- Use a monospace font by default.
- Current prototype uses Cairo `monospace` at `14px`.
- Future renderer should support configurable font family, size, line height, and fallback fonts.
- Correct shaping and glyph fallback matter for modern terminal behavior.

### UI Text

- GTK-native typography is acceptable for initial settings, menus, and window UI.
- Avoid building a large custom design system before the terminal engine is viable.

## Terminal Surface

The terminal surface is the primary UI. It should prioritize:

- Low latency keyboard feedback.
- Stable cell metrics.
- Accurate cursor rendering.
- Correct selection and copy behavior.
- Smooth scrolling.
- Correct placement of images relative to the cell grid.

## Renderer Expectations

The long-term renderer must compose:

1. Background.
2. Cell backgrounds.
3. Image/graphics placements.
4. Text glyphs.
5. Text decorations: underline, strikethrough, hyperlinks.
6. Selection overlay.
7. Cursor.
8. IME/preedit UI if needed.

Do not treat text rendering and image rendering as separate unrelated overlays. They must share the same grid and viewport model.

## Image Display Requirements

For Kitty-class image support:

- Images must anchor to terminal grid positions.
- Images must survive redraws and scrolling according to protocol semantics.
- Images need decode/cache lifecycle management.
- Renderer should use textures for performance.
- Image placement must handle z-order relative to text and backgrounds.

## Window Layout

### Current

- Single GTK4 window.
- Single terminal surface.
- Default size: `900x620`.

### Future

- Tabs may sit above the terminal surface.
- Splits/panes may divide the main terminal area.
- Settings should be native-feeling and not block terminal rendering work.
- Search UI should overlay or attach without disturbing terminal grid metrics.

## Interaction Conventions

- Keyboard input should be sent to the terminal by default when the terminal has focus.
- Terminal focus should be clear visually.
- Copy/paste should follow terminal conventions.
- Mouse support must respect terminal mouse tracking modes.
- Future shortcuts should avoid breaking common shell/TUI expectations.

## Icons

No icon library is chosen yet. Use GTK-native icons or minimal symbolic icons only when needed. Avoid adding a large icon dependency before tabs/settings/search exist.
