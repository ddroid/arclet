# Backend And Renderer Recommendation Report

## Executive Summary

Arclet should keep VTE only as a temporary scaffold and continue toward a custom backend. The best near-term path is:

1. **Terminal core:** Continue prototyping with `libghostty-vt`, but place it behind a viability gate because its public consumer API may not yet expose enough Kitty graphics/image data.
2. **PTY:** Keep `portable-pty`; fix resize by retaining the PTY master handle and calling `MasterPty::resize()` whenever the terminal grid changes.
3. **Renderer prototype:** Move from `DrawingArea + Cairo row text` to a custom GTK4 widget using `snapshot()` plus a renderer-independent scene model as the next safe step.
4. **Future GPU renderer:** Prototype **GTK4 `GLArea` + OpenGL** as the first GPU renderer, not `wgpu`, because it has direct GTK integration, proven terminal-emulator precedent, and straightforward texture support for future images.
5. **Text stack:** Use **Pango/GTK Snapshot** for the correctness-first renderer, then prototype **cosmic-text + swash glyph atlas uploaded to OpenGL textures** for the longer-term GPU renderer.
6. **Image protocol:** Do not depend solely on `libghostty-vt` for Kitty image rendering until image placements and pixel data are exposed through its render-state API. Prepare an Arclet-owned image registry/placement/cache layer.

The best fit for Arclet's vision is not “use Ghostty as the whole backend.” It is “use Ghostty/libghostty-vt as the terminal emulation core if it remains viable, while Arclet owns PTY, input integration, rendering, image cache, UI, and lifecycle.”

## Project Vision Constraints

Arclet's documented direction is:

- Rust 2024.
- GTK4 native Linux UI.
- VTE is temporary scaffold only.
- Custom terminal path must become the product backend.
- Renderer must handle styled text, cursor, selection, scrollback, and future grid-positioned images.
- PTY, terminal state, input, renderer, and UI shell must remain separate.
- Cairo row drawing is prototype-only.

## Research Sources

- Kitty overview: https://sw.kovidgoyal.net/kitty/overview/
- Kitty graphics protocol: https://sw.kovidgoyal.net/kitty/graphics-protocol/
- Kitty keyboard protocol: https://sw.kovidgoyal.net/kitty/keyboard-protocol/
- Ghostty repository roadmap/status: https://github.com/ghostty-org/ghostty
- Ghostty VT reference: https://ghostty.org/docs/vt/reference
- Ghostling minimal libghostty terminal: https://github.com/ghostty-org/ghostling
- `libghostty-vt` docs: https://docs.rs/libghostty-vt/latest/libghostty_vt/
- `libghostty-vt` render state docs: https://docs.rs/libghostty-vt/latest/libghostty_vt/struct.RenderState.html
- Ghostty issue about image exposure: https://github.com/ghostty-org/ghostty/issues/12111
- GTK4 `GLArea`: https://docs.gtk.org/gtk4/class.GLArea.html
- gtk-rs `GLArea`: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.GLArea.html
- GTK4 `Snapshot`: https://docs.gtk.org/gtk4/class.Snapshot.html
- Pango rendering pipeline: https://docs.gtk.org/Pango/pango_rendering.html
- PangoCairo: https://docs.gtk.org/PangoCairo/pango_cairo.html
- `cosmic-text`: https://docs.rs/cosmic-text/latest/cosmic_text/
- `swash`: https://github.com/dfrg/swash
- `harfbuzz_rs`: https://github.com/harfbuzz/harfbuzz_rs
- `portable-pty` `MasterPty`: https://docs.rs/portable-pty/latest/portable_pty/trait.MasterPty.html
- `wgpu`: https://wgpu.rs/

## Do We Really Need Ghostty?

### Short Answer

We do not need Ghostty as a complete terminal application backend. We probably do want `libghostty-vt` as the terminal emulation core if its build and API viability hold.

### Why Ghostty/libghostty-vt Is Attractive

- **Proven terminal behavior:** Ghostty itself claims broad xterm/modern terminal compatibility and millions of users/machines.
- **Scope alignment:** `libghostty-vt` focuses on parsing terminal sequences and maintaining terminal state, exactly the part Arclet should not reinvent casually.
- **Render-state API:** `RenderState` provides a snapshot model with dirty tracking, colors, cursor state, row/cell iteration, graphemes, and styles.
- **Input support:** Ghostling documents keyboard modifier support, Kitty keyboard protocol, focus reporting, mouse tracking, and mouse reporting formats as core libghostty features.
- **Threading model is explicit:** `libghostty-vt` objects are `!Send + !Sync`, requiring ownership on one thread and communication through channels. This matches Arclet's GTK main-thread invariant.
- **Better than building a parser from scratch:** Terminal emulation correctness is a deep compatibility pit. Ghostty gives Arclet a much stronger starting point.

### Why Ghostty/libghostty-vt Is Risky

- **Native build fragility:** Current project notes say `libghostty-vt-sys` may need network downloads from `deps.files.ghostty.org`. This is not acceptable as an unbounded long-term build risk.
- **API maturity:** `libghostty-vt` is young and the Ghostty roadmap says libghostty is being broken down progressively, starting with `libghostty-vt`.
- **Image exposure gap:** A 2026 Ghostty issue states Ghostty processes Kitty graphics internally, but render-state C API does not expose image data, placement positions, dimensions, or pixel buffers to consumers. That means Arclet may not be able to render inline images by relying on the current public API alone.
- **Not the whole terminal:** libghostty does not give Arclet tabs, splits, config, session management, search UI, rendering, GTK integration, or PTY lifecycle. Arclet must own those.

### Recommendation

Use `libghostty-vt` behind a `TerminalCore` boundary, but do not hard-code Arclet's entire future around Ghostty until these gates pass:

1. **Build gate:** Ghostty backend builds reproducibly without surprise network failure.
2. **Runtime gate:** Shell, resize, input, alternate screen, truecolor, scrollback, and shutdown work.
3. **API gate:** Render-state exposes enough cell/style/cursor/dirty data for Arclet's renderer.
4. **Image gate:** Either libghostty-vt exposes Kitty graphics placements/pixel data, or Arclet implements its own Kitty graphics parser/registry layered beside the terminal core.
5. **Maintenance gate:** The API is stable enough that upgrades are acceptable.

If gate 4 fails, Ghostty can still be useful for VT text/state, but Arclet needs an Arclet-owned Kitty graphics layer.

## Backend Options

| Option | Pros | Cons | Fit |
| --- | --- | --- | --- |
| VTE only | Stable, already works, mature GTK integration | Violates custom backend vision; limited Arclet control over rendering/images | Temporary scaffold only |
| libghostty-vt | Strong terminal correctness, modern protocol support, render-state API, Kitty keyboard support | Build/API/image exposure risk; no renderer/windowing | Best current custom core candidate |
| Alacritty terminal core | Rust, proven fast terminal core | Integration and licensing/API fit need research; less Kitty image focus historically | Backup candidate |
| WezTerm internals | Very feature-rich Rust terminal | Large project, not primarily packaged as simple embeddable core | Backup/research candidate |
| Write custom parser/state | Maximum control | Very high correctness risk and long timeline | Not recommended |

## Renderer Technology Options

### Option 1 — GTK4 `DrawingArea` + Cairo

**Pros**

- Already works in the prototype.
- Very easy to integrate with GTK.
- Good for first visible proof-of-life.
- Simple debugging.

**Cons**

- Per-row `show_text` does not scale to a real terminal renderer.
- Weak fit for GPU image textures.
- Hard to build efficient glyph atlas/dirty-region rendering.
- Current path ignores styles, cursor, selection, scrollback composition, image z-order.

**Recommendation**

Keep only as a disposable fallback/proof renderer. Do not build image support on this.

### Option 2 — GTK4 custom widget `snapshot()` / GSK render nodes

**Pros**

- Native GTK4 rendering path.
- `GtkSnapshot` can append colors, layouts, textures, and render nodes.
- Good bridge from Cairo prototype to scene-based rendering.
- Integrates cleanly with GTK widget lifecycle.
- `append_texture` and `append_scaled_texture` are directly relevant to future image placements.
- Less complexity than owning an OpenGL renderer immediately.

**Cons**

- Less low-level control than direct OpenGL.
- Performance for massive terminal output needs measurement.
- A terminal renderer may create many nodes if implemented naively.
- Advanced glyph atlas control is not as direct.

**Recommendation**

Best **next renderer prototype** for correctness-first scene extraction. Use this after creating an `ArcletRenderScene` model.

### Option 3 — GTK4 `GLArea` + OpenGL

**Pros**

- Official GTK widget for OpenGL drawing.
- Direct texture support for glyph atlas and images.
- Matches proven architecture used by fast terminals such as Kitty/Ghostty/Alacritty-style renderers.
- GTK docs say the render callback has a current `GdkGLContext` and framebuffer sized to allocation.
- Easy conceptual model: draw cell backgrounds, image quads, glyph atlas quads, selection overlay, cursor.
- Better fit than Cairo for Kitty graphics.

**Cons**

- Requires shader/buffer/texture management.
- Requires OpenGL expertise and careful lifecycle handling on realize/unrealize.
- Text rendering must be solved separately.
- Linux OpenGL driver issues may exist.

**Recommendation**

Best **GPU prototype renderer technology** for Arclet. It is the most practical future renderer target given GTK4 and image-texture goals.

### Option 4 — `wgpu` embedded in GTK4

**Pros**

- Modern Rust graphics abstraction.
- Portable over Vulkan/Metal/DX12/OpenGL ES/WebGPU.
- Excellent long-term abstraction if Arclet later expands beyond Linux.
- Good ecosystem for GPU buffers/textures.

**Cons**

- GTK4 integration is not as direct as `GLArea`.
- Surface/window-handle integration adds complexity.
- More moving parts than OpenGL for a Linux GTK4-first terminal.
- Could distract from terminal correctness.

**Recommendation**

Do not choose as the first GPU prototype. Revisit after OpenGL/GLArea proves renderer architecture and if portability becomes important.

### Option 5 — Vulkan directly

**Pros**

- Modern explicit GPU API.
- Strong performance and control.

**Cons**

- Too much complexity for the current stage.
- GTK integration and text/image pipeline complexity are high.
- Slower iteration.

**Recommendation**

Not recommended for now.

## Text Shaping And Glyph Rendering Options

### Option 1 — Pango/Cairo

**Pros**

- Mature Linux/GTK text stack.
- Handles Unicode shaping, font fallback, and system font integration well.
- Simple to use for a correctness-first renderer.
- Pango rendering pipeline explicitly covers itemization, shaping, layout, and rendering.
- PangoCairo can render shaped layouts through Cairo.

**Cons**

- CPU rendering path.
- Not ideal for high-throughput GPU terminal rendering.
- Uploading Cairo-rendered output to GPU textures can become inefficient if done per frame or per row.
- Less control over glyph atlas strategy.

**Fit**

Excellent for correctness-first prototype, weak as final high-performance renderer.

### Option 2 — Pango over GL / Pango-to-texture

**Pros**

- Retains mature shaping/fallback.
- Can render text to Cairo image surfaces and upload to GL textures.
- Practical stepping stone from GTK text correctness to GPU composition.

**Cons**

- Texture upload cost can be high.
- Row/string texture caching gets complicated with rapidly changing terminal content.
- Not a clean glyph-atlas design.
- Harder to compose per-cell colors/styles efficiently.

**Fit**

Useful intermediate experiment, not preferred as final renderer.

### Option 3 — HarfBuzz + custom glyph atlas

**Pros**

- Industry-standard shaping engine.
- Full control over glyph atlas, batching, shaders, cache eviction.
- Can support ligatures, complex scripts, and advanced OpenType features.
- Best theoretical long-term performance/control.

**Cons**

- HarfBuzz does not draw; rasterization, font discovery, fallback, caching, layout, emoji/color glyphs must be solved separately.
- Highest engineering cost.
- Easy to get wrong for non-Latin text.

**Fit**

Strong final architecture candidate only if Arclet is ready to own a lot of text stack complexity.

### Option 4 — `cosmic-text`

**Pros**

- Rust-native advanced text handling.
- Provides shaping, font discovery, font fallback, layout, rasterization, and editing abstractions.
- Uses `fontdb` for discovery, `harfrust` for shaping, optional `swash` for rasterization.
- `SwashCache` can rasterize glyphs into images/pixels suitable for upload into a GPU atlas.
- Good balance of control and ease-of-use.

**Cons**

- Additional dependency stack.
- Terminal-specific grid shaping still needs careful design.
- Need benchmarking for high-throughput terminal workload.
- GTK-native font behavior may differ from Pango unless configured carefully.

**Fit**

Best long-term text stack candidate for an Arclet-owned GPU renderer.

### Option 5 — `swash` directly

**Pros**

- High-performance font introspection, shaping, scaling, and glyph rendering.
- Supports OpenType, variable fonts, complex scripts, color emoji formats, subpixel rendering.
- Designed to be unopinionated about resource management and composition.
- Thread-friendly and allocation-conscious.

**Cons**

- Explicitly not a text layout or composition library.
- More work than `cosmic-text` because Arclet must provide layout/fallback/render composition.
- Better as a lower-level component than the primary text API.

**Fit**

Excellent lower-level component; use through `cosmic-text` first unless deeper control is needed.

### Option 6 — GTK `Snapshot.append_layout`

**Pros**

- Native GTK4 text layout rendering.
- Good fit for a custom widget and correctness-first prototype.
- Integrates with `PangoLayout`.

**Cons**

- Not a custom GPU glyph atlas.
- May be too node/layout heavy if used per cell naively.
- Needs careful batching by runs/rows.

**Fit**

Best immediate replacement for direct Cairo row drawing.

## Recommended Renderer Roadmap

### Phase R1 — Scene Model First

Create `ArcletRenderScene` independent of drawing backend:

- viewport size,
- cell metrics,
- visible rows/cells,
- resolved foreground/background colors,
- text graphemes,
- style flags,
- cursor,
- selection ranges,
- scroll offset,
- image placements,
- dirty rows/regions.

Use `libghostty-vt::RenderState` where possible for rows, cells, styles, cursor, colors, and dirty tracking.

### Phase R2 — Correctness Renderer With GTK Snapshot + Pango

Implement a custom GTK widget that renders scene layers in order:

1. terminal background,
2. cell backgrounds,
3. negative-z image placeholders later,
4. text layouts,
5. decorations,
6. selection overlay,
7. cursor.

Use Pango/GTK Snapshot for text. This gives correctness and GTK integration before GPU complexity.

### Phase R3 — GLArea Prototype

Prototype a separate `GLArea` renderer consuming the same `ArcletRenderScene`:

- draw background/cell quads,
- draw image texture quads,
- rasterize glyphs using `cosmic-text`/`swash`,
- upload glyphs to an atlas,
- batch glyph quads by texture/style,
- render cursor/selection overlays.

### Phase R4 — Image Pipeline

Implement Arclet image subsystem independent of renderer:

- Kitty command parser/integration point,
- image registry,
- placement registry,
- decode worker,
- texture cache interface,
- memory quota/eviction,
- renderer-provided texture upload.

If `libghostty-vt` later exposes image data through render state, adapt the image subsystem to consume that data instead of duplicating parsing.

## Recommended Prototype Renderer Technology

The recommended prototype renderer sequence is:

1. **Immediate:** GTK4 custom widget + `snapshot()` + Pango layouts.
2. **GPU prototype:** GTK4 `GLArea` + OpenGL + `cosmic-text`/`swash` rasterized glyph atlas.
3. **Future optional:** Re-evaluate `wgpu` after the OpenGL scene/glyph/image model is proven.

This sequence best matches Arclet's vision because it:

- preserves GTK-native integration,
- avoids premature GPU complexity,
- creates a renderer-independent scene model,
- leaves a direct path to textures for Kitty images,
- keeps text rendering correctness ahead of performance tuning,
- and does not bind image architecture to Cairo.

## Ghostty API Stability And Image Exposure

### What Looks Good

- `libghostty-vt` has safe Rust bindings.
- Docs include `Terminal`, `RenderState`, render row/cell iteration, dirty tracking, colors, cursor state.
- Ghostling documents features inherited from libghostty-vt: resize with reflow, 24-bit and 256-color, styles, graphemes, Kitty keyboard, Kitty graphics protocol, mouse tracking/reporting, focus reporting.
- Thread-safety model is explicit: objects are `!Send + !Sync` and should be managed on one thread.

### What Looks Risky

- `libghostty-vt` is still part of a broader libghostty extraction effort.
- The public API may lag Ghostty GUI capabilities.
- The image exposure issue is directly relevant: consumers may not be able to render inline images even if Ghostty internally processes Kitty graphics.

### Recommendation

Keep Ghostty as the preferred terminal core candidate, but do not make the image architecture depend on Ghostty render-state image exposure until verified.

Arclet should define a `TerminalCore` abstraction with methods like:

- feed PTY bytes,
- resize terminal state,
- update render snapshot,
- encode terminal responses,
- expose mode/input state,
- expose render cells/styles/cursor/dirty state,
- optionally expose graphics/image events.

Then Arclet can replace or supplement `libghostty-vt` if needed.

## GTK/wgpu/GLArea Viability

### GTK `Snapshot`

GTK `Snapshot` assists in creating `GskRenderNode`s for widgets. It can append color nodes, Cairo nodes, Pango layouts, textures, and scaled textures. This is ideal for a correctness-first custom GTK widget because it uses GTK's normal render pipeline.

### GTK `GLArea`

GTK `GLArea` is a real OpenGL drawing widget. Its render callback is called when ready to draw, with a current `GdkGLContext`; the viewport is already set to the allocation size. It supports realize/unrealize lifecycle for creating and cleaning GL resources.

This makes it viable for Arclet's GPU renderer.

### `wgpu`

`wgpu` is a safe Rust graphics library based on WebGPU and can run over Vulkan, Metal, DX12, OpenGL ES, and WebGPU. It is attractive long-term, but GTK4 integration is not as straightforward as `GLArea` for a Linux-first GTK app.

### Recommendation

Use GTK `Snapshot` first, `GLArea` next, and defer `wgpu`.

## PTY Resize Problem

### Current Problem

The Ghostty backend currently recalculates rows/cols and calls `terminal.resize(...)`, but it does not resize the underlying PTY master. Real TUI apps query terminal size from the kernel PTY and receive resize notification through `SIGWINCH`. If only terminal state is resized, apps like `vim`, `less`, `htop`, and `stty size` remain wrong.

### Correct Mechanism

`portable-pty::MasterPty::resize(size: PtySize)` exists specifically for this. The docs state that it informs the kernel and child process that the window resized, updates kernel winsize information, and generates the signal for the child to notice and update its state.

### Required Code Architecture Change

The custom backend must retain the PTY master handle, not only reader/writer handles.

Recommended structure:

- `PtySession`
  - owns `Box<dyn MasterPty + Send>` or a backend-specific master handle,
  - owns writer handle,
  - owns child process handle,
  - owns reader thread join/shutdown channel,
  - exposes `resize(cols, rows, pixel_width, pixel_height)`.

Resize flow:

1. GTK allocation changes.
2. Cell metrics convert pixels to cols/rows.
3. If cols/rows changed:
   - call `Terminal::resize(cols, rows, pixel_width, pixel_height)`,
   - call `master.resize(PtySize { rows, cols, pixel_width, pixel_height })`,
   - queue render update,
   - record/log resize errors.

### Important Details

- Debounce rapid resize if necessary, but do not skip final resize.
- Compute rows/cols from actual font/cell metrics, not fixed constants long-term.
- Update terminal state and PTY size in the same logical operation.
- Pixel dimensions should use the actual terminal surface size or cell-derived pixel size consistently.
- Do not block GTK main loop.
- Treat resize errors as real errors, not silent `_ =` operations.
- Add smoke test with `stty size` and `vim`/`less` after implementation.

### Minimal Fix Direction

For the current prototype, `TerminalView` should store the PTY master handle in addition to writer. In `resize()`, after `terminal.resize(...)`, call `master.resize(...)` with the new `PtySize`.

Do not implement this as a one-off only inside rendering. Move toward a dedicated PTY module because resize belongs to PTY lifecycle, not drawing.

## Final Recommendations

### Backend

- Keep VTE as a temporary scaffold only.
- Continue with `libghostty-vt` as the preferred terminal core candidate.
- Add a viability gate before treating Ghostty as irreversible architecture.
- Prepare an Arclet-owned Kitty graphics layer because Ghostty image exposure is currently uncertain.

### Renderer

- Next prototype: GTK custom widget + `Snapshot` + Pango for correctness.
- GPU prototype: GTK `GLArea` + OpenGL.
- Text stack for GPU: `cosmic-text` + `swash` first; only drop to direct HarfBuzz if necessary.
- Do not choose `wgpu` until after GLArea proves scene/glyph/image architecture.

### PTY

- Keep `portable-pty`.
- Refactor PTY ownership into a dedicated module.
- Fix resize by retaining `MasterPty` and calling `resize()` on grid changes.
- Add smoke tests around `stty size`, alternate-screen apps, process exit, and high-output behavior.

### Immediate Next Work

1. Update context docs to state VTE is temporary scaffold, not permanent fallback.
2. Add `PtySession` abstraction and fix PTY resize propagation.
3. Introduce `ArcletRenderScene` using `libghostty-vt::RenderState` where possible.
4. Replace direct Cairo row loop with GTK Snapshot/Pango correctness renderer.
5. Prototype `GLArea` renderer behind a feature flag.
6. Write a dedicated Kitty graphics architecture spec before implementing images.
