# RustedUI — Architecture, Zero-GC FFI Protocol & Cross-Platform ABI

RustedUI separates **layout, styling, hit-testing, and command generation** (written in safe, zero-GC Rust) from **GPU texture drawing and SMAPI lifecycle hooks** (written in C# for Stardew Valley 1.6 / MonoGame 3.8).

---

## 1. End-to-End Frame Pipeline

```mermaid
sequenceDiagram
    participant SMAPI as SMAPI ModEntry (C#)
    participant View as RustedView (C#)
    participant Bridge as rusted_bridge (.so / .dll)
    participant Core as rusted_core::UiTree (Rust)
    participant GPU as MonoGame SpriteBatch

    Note over SMAPI,Core: 1. UpdateTicked (60 Hz Input & Events)
    SMAPI->>View: UpdateInput(uiMouseX, uiMouseY, leftDown, scrollDelta)
    View->>Bridge: rusted_tree_update_input(tree, x, y, down, scroll, &eventSliceView)
    Bridge->>Core: hit_test_recursive() -> update hover/active/slider/scroll -> push UiEvents
    Core-->>View: EventSliceView (Ptr + Len over reusable Rust Vec<UiEvent>)
    View->>SMAPI: Dispatch OnAction callbacks & play vanilla sounds

    Note over SMAPI,GPU: 2. RenderedHud / IClickableMenu.draw (60-120 FPS Render)
    SMAPI->>View: Draw(SpriteBatch)
    View->>Bridge: rusted_tree_render(tree, viewportW, viewportH, &renderSliceView)
    Bridge->>Core: Taffy compute_layout_with_measure() + snap_and_emit_recursive() + emit_floating_tooltip()
    Core-->>View: RenderSliceView (Ptr + Len over reusable Rust Vec<RenderCommand>)
    View->>GPU: RustedRenderer iterates ReadOnlySpan<RenderCommand> (0 Bytes GC!)
```

---

## 2. Memory & Performance Guarantees

- **0 Bytes Managed GC Per Frame**:
  - `RenderSliceView` and `EventSliceView` pass raw pointers and lengths to pre-allocated, reusable `Vec<RenderCommand>` and `Vec<u8>` buffers owned by Rust's `UiTree`.
  - C# wraps these pointers in stack-only `ReadOnlySpan<RenderCommand>` and `ReadOnlySpan<UiEvent>` with `[SuppressGCTransition]` P/Invoke attributes.
- **Hardware `ScissorRectangle` Clipping**:
  - `<scroll-view>` emits `PushClipRect` and `PopClipRect` commands.
  - `RustedRenderer.cs` flushes the current `SpriteBatch`, intersects the clip rectangle with `GraphicsDevice.ScissorRectangle` using a stack for nested clips, begins a `RasterizerState(ScissorTestEnable = true, CullNone)` pass, and restores `RasterizerState.CullNone` on `PopClipRect`.
- **Cross-Platform Native Binary Resolution (`NativeResolver.cs`)**:
  - **Linux Native (`linux-x64`)**: Loads `runtimes/linux-x64/native/librusted_bridge.so`.
  - **Windows & Linux Proton/Wine (`win-x64`)**: Loads `runtimes/win-x64/native/rusted_bridge.dll`, compiled via `cargo xwin` with `RUSTFLAGS="-C target-feature=+crt-static"` (.cargo/config.toml) so `vcruntime140.dll` is statically linked and never missing inside Wine/Proton sandboxes.
