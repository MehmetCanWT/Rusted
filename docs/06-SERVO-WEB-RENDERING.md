# Rusted — Servo Web Rendering Integration Guide

> **Status:** Architecture and implementation plan (not yet implemented)
>
> This document describes how to add an optional, embedded Servo browser surface to Rusted. It does not claim that the current Rusted renderer already supports Servo.

## 1. Goal

Add a real browser-rendering path for HTML, CSS and JavaScript while keeping the existing RustedUI renderer intact.

Rusted currently has a deliberately small native UI pipeline:

- `ui-tsx/` compiles React/TSX ahead of time into RustedUI markup.
- `rusted-core` parses markup, stores the UI tree and computes layout with Taffy.
- `rusted-bridge` exposes the native UI engine through a C ABI.
- `RustedView` calls the native bridge and forwards input.
- `RustedRenderer` translates render commands into MonoGame `SpriteBatch` calls.

Servo should be an **optional second rendering backend**, not a replacement for `rusted-core`, Taffy or the current renderer.

### Target use cases

Use Servo when a mod needs browser features such as:

- Modern HTML and CSS layout that is not part of RustedUI's markup model.
- JavaScript-driven interfaces and existing web applications.
- Web fonts, SVG and browser-native form controls.
- A page loaded from a local asset or an explicitly permitted URL.

Keep ordinary HUDs, inventory widgets, tooltips and small native menus on the existing RustedUI path. A full browser engine has a substantially larger memory, startup, binary-size and security footprint.

## 2. Current architecture and integration boundary

The current repository is not a browser. TSX is compiled at build time; it is not evaluated by a browser runtime in-game.

Current path:

```text
TSX / HTML markup
       |
       v
RustedView (C#)
       |
       v
rusted-bridge (C ABI)
       |
       v
rusted-core (SlotMap + Taffy)
       |
       v
RenderSliceView
       |
       v
RustedRenderer -> MonoGame SpriteBatch
```

Proposed optional path:

```text
C# SMAPI / MonoGame
       |
       | URL, size, mouse, keyboard, wheel, focus
       v
RustedWebView (C# owner / adapter)
       |
       | stable C ABI; opaque handle + commands
       v
rusted-web (new Rust crate; Servo embedder)
       |
       +--> Servo WebView / DOM / CSS / JavaScript
       |
       +--> Servo RenderingContext / WebRender
       |
       v
RGBA frame surface (initial implementation)
       |
       | frame-ready + dimensions + pixel bytes
       v
C# Texture2D upload
       |
       v
RustedWebRenderer -> MonoGame SpriteBatch
```

The existing native UI and the browser surface can be composed in one menu: draw the web texture at a configured rectangle, then draw RustedUI overlays, tooltips or native controls above it.

## 3. Recommended crate layout

Do not add Servo to `rusted-core`. That crate should remain a small, deterministic layout and command-generation engine.

Add a separate crate:

```text
crates/
  rusted-core/       # Existing Taffy-based native UI engine
  rusted-bridge/     # Existing RustedUI C ABI
  rusted-web/        # New optional Servo integration
    Cargo.toml
    src/
      lib.rs
      runtime.rs
      view.rs
      rendering.rs
      input.rs
      ffi.rs
```

Responsibilities:

| Module | Responsibility |
|---|---|
| `runtime.rs` | Own Servo instance, event-loop wake-up and lifecycle |
| `view.rs` | Create, navigate, resize and destroy Servo WebViews |
| `rendering.rs` | RenderingContext, frame capture and frame metadata |
| `input.rs` | Convert Rusted/MonoGame input into Servo input events |
| `ffi.rs` | Stable C ABI; no Servo Rust types cross the ABI |

Add `crates/rusted-web` to the workspace only when the Servo toolchain and target builds have been validated. Keep it behind an opt-in Cargo feature or a separate build artifact so users who do not need web content do not have to ship Servo.

## 4. Servo embedding lifecycle

The current Servo API exposes a `Servo` instance, `WebView`, `WebViewBuilder`, `EventLoopWaker`, `WebViewDelegate` and `RenderingContext`.

The high-level lifecycle is:

1. Create an event-loop waker. Its `wake` implementation must arrange for Servo's `spin_event_loop` to run.
2. Build one Servo runtime using `ServoBuilder`.
3. Create a RenderingContext for the web surface.
4. Create a WebView with `WebViewBuilder`, attach a `WebViewDelegate` and configure its viewport.
5. Load a URL or local document.
6. Forward host input to the WebView and call `Servo::spin_event_loop` so Servo can process it.
7. When a new frame is ready, call `WebView::paint`, present/capture the rendered result and publish the frame to the host.
8. On disposal, close the WebView and release the rendering context and Servo resources.

The current Servo crate documentation specifically describes this sequence. The embedding chapter in the Servo Book also warns that its embedding documentation is still a work in progress. Treat the Servo API as version-sensitive and pin a tested revision.

### API sketch (illustrative, not drop-in code)

The following is a lifecycle sketch, not a complete compilable integration. Exact builder arguments, delegate callbacks and input event constructors must be taken from the exact Servo revision pinned by Rusted.

```rust
// Pseudocode: keep all Servo-owned objects behind the Rust boundary.
struct RustedWebRuntime {
    servo: Servo,
    // Keep the rendering context and WebView alive for the view's lifetime.
}

impl WebViewDelegate for RustedWebDelegate {
    fn notify_new_frame_ready(&self, /* callback args for pinned Servo version */) {
        // Signal the Rusted web runtime that a frame should be painted.
        // Do not call back into C# or MonoGame from an arbitrary Servo thread.
    }
}

// On the Servo owner/event-loop thread:
// 1. Build Servo with an EventLoopWaker.
// 2. Create OffscreenRenderingContext (or a custom context).
// 3. Build WebView with WebViewBuilder.
// 4. webview.load(url).
// 5. servo.spin_event_loop().
// 6. On frame-ready: webview.paint(); capture/publish the frame.
```

Do not copy old Servo examples that use the historic `WindowMethods` / `WindowEvent` API without checking their revision. The current public API is centered around `ServoBuilder`, `WebViewBuilder`, `WebViewDelegate` and `RenderingContext`.

## 5. Rendering into MonoGame

This is the most important integration decision.

Servo's standard `WindowRenderingContext` targets a raw-window-handle window using Surfman/OpenGL. Rusted runs inside Stardew Valley and receives a MonoGame `SpriteBatch`; Rusted does not own the game's native window or graphics context.

Therefore, do not assume that Servo can render directly into the game's backbuffer.

### Phase 1 — Offscreen RGBA upload (recommended first milestone)

Use Servo's `OffscreenRenderingContext` or a custom offscreen `RenderingContext`. After Servo paints a frame, obtain an RGBA image using the rendering context's image-readback capability (`read_to_image` in the current trait).

Publish a frame record containing:

- Width and height in physical pixels.
- Pixel format (explicitly RGBA8).
- Row stride / bytes per row.
- Frame sequence number.
- Pixel buffer length.

On the C# side, create or resize a `Texture2D` on the MonoGame graphics thread, upload the bytes with `Texture2D.SetData`, and draw the texture using `SpriteBatch.Draw`.

Illustrative C# rendering shape:

```csharp
// Pseudocode: Texture2D creation, resize and SetData must run
// on the MonoGame graphics thread.
if (webFrame.IsNew)
{
    EnsureWebTexture(webFrame.Width, webFrame.Height);
    _webTexture.SetData(webFrame.RgbaPixels);
}

batch.Draw(
    _webTexture,
    destinationRectangle,
    sourceRectangle: null,
    Color.White
);
```

**Trade-off:** readback plus CPU-to-GPU texture upload is easy to reason about and is a good correctness milestone, but it is not zero-copy. It can be expensive for large surfaces or frequently changing pages. Do not advertise this path as zero-GC or zero-copy until profiling proves it.

Use a bounded double/triple-buffer strategy. Never let the Servo producer overwrite a pixel buffer while C# is uploading it. Reuse buffers and avoid allocating a new managed byte array for every frame. Handle row stride and vertical orientation explicitly.

### Phase 2 — GPU texture interop (optimization)

Only after the CPU path works, investigate sharing a GPU texture between Servo's OpenGL context and MonoGame's graphics backend.

This is platform- and backend-dependent:

- MonoGame WindowsDX commonly uses Direct3D; Servo's standard contexts are OpenGL-oriented.
- MonoGame DesktopGL also uses OpenGL, but sharing contexts, devices and texture ownership still requires explicit context creation and synchronization.
- Linux native and Proton/Wine must be tested independently.
- A texture handle from one graphics API is not automatically a valid texture in another API.

A correct interop implementation needs a documented shared-device/context strategy, synchronization/fences, format agreement, lifetime management and a fallback path. If a stable cross-backend path is not available, retain the RGBA upload implementation.

Do not try to inject Servo's OpenGL calls into an active MonoGame SpriteBatch pass.

## 6. Threading and frame ownership

Servo has its own event processing and rendering work. MonoGame graphics resources must be manipulated on the game/graphics thread.

Recommended ownership model:

- One Rust-owned Servo runtime thread owns `Servo`, `WebView` and their rendering context unless the pinned Servo API explicitly documents another safe arrangement.
- C# calls enqueue commands: create, navigate, resize, mouse, wheel, keyboard and close.
- Servo's event-loop waker wakes the owner thread; that thread calls `spin_event_loop`.
- A completed frame is published into a bounded frame queue or a double-buffered shared-memory slot.
- The C# game thread consumes the newest complete frame and performs `Texture2D` upload/draw.
- Never pass Rust references, trait objects, Servo handles or C++/OpenGL context pointers through the C ABI.
- Never call MonoGame APIs from Servo callbacks or the Servo worker thread.

The exact thread model must respect the Send/Sync constraints of the Servo revision in use. If Servo objects are not Send, create and use them entirely on the dedicated owner thread; send only plain command data across the channel.

## 7. C ABI design

Keep the ABI small, versioned and independent of Servo's internal types. Example contract:

```c
typedef struct RustedWebView RustedWebView;

typedef struct {
    uint32_t width;
    uint32_t height;
    uint32_t stride_bytes;
    uint32_t pixel_format;   // 1 = RGBA8
    uint64_t frame_sequence;
    const uint8_t* pixels;   // borrowed until next acquire/release boundary
    size_t byte_len;
} RustedWebFrame;

uint32_t rusted_web_abi_version(void);

RustedWebView* rusted_web_view_create(
    uint32_t width,
    uint32_t height
);

int32_t rusted_web_view_navigate(
    RustedWebView* view,
    const uint8_t* url_utf8,
    size_t url_len
);

int32_t rusted_web_view_resize(
    RustedWebView* view,
    uint32_t width,
    uint32_t height
);

int32_t rusted_web_view_send_pointer(
    RustedWebView* view,
    /* normalized event fields */
);

int32_t rusted_web_view_send_keyboard(
    RustedWebView* view,
    /* normalized key event fields */
);

int32_t rusted_web_view_acquire_frame(
    RustedWebView* view,
    RustedWebFrame* out_frame
);

void rusted_web_view_release_frame(
    RustedWebView* view,
    uint64_t frame_sequence
);

void rusted_web_view_destroy(RustedWebView* view);
```

This is an ABI proposal, not implemented exports. Define exact pointer ownership, error codes, UTF-8 rules, thread-safety and frame lifetime before implementing it. Do not expose a pointer to a temporary Rust `Vec`. The frame must remain valid until the documented release operation completes.

Use opaque handles and `#[repr(C)]` structs only. Catch Rust panics at every exported FFI boundary; no unwind may cross into C#.

## 8. C# integration points

Add a separate adapter rather than changing `RustedView` into a browser object:

```text
src/Rusted.Smapi/
  Web/
    RustedWebView.cs       # Owns native handle and public web-view API
    RustedWebRenderer.cs   # Uploads frame and draws Texture2D
    RustedWebInput.cs      # Converts SMAPI/MonoGame input
    RustedWebNative.cs     # P/Invoke declarations
```

Suggested public API shape:

```csharp
using var web = new RustedWebView(width: 960, height: 540);
web.Navigate("https://example.com");

// In the menu/HUD draw method:
web.UpdateFrame();        // Consume a completed frame on the game thread.
web.Draw(spriteBatch, destinationRectangle);

// Forward input only when the pointer is inside the web rectangle.
web.SendPointerMove(x, y);
web.SendWheel(deltaX, deltaY);
web.SendKey(key, isDown);
```

The names above are a proposed C# API. Implement them only after the native ABI is stable.

### Input mapping

Map coordinates from the displayed destination rectangle into the web surface's CSS pixel coordinate space. Account for UI scale, DPI and the current viewport size.

Forward at least:

- Pointer move, button down and button up.
- Wheel / trackpad scrolling.
- Key down, key up and text/IME input.
- Focus gained/lost.
- Surface resize.

When a web view owns pointer capture or keyboard focus, avoid also dispatching the same event to RustedUI underneath it. Preserve Escape/menu-close policy at the host level.

## 9. Build and dependency strategy

Servo is much larger than the existing `taffy`, `slotmap` and `quick-xml` dependencies. Its toolchain, native dependencies, resource files and build time need to be treated as a separate product concern.

Recommended rollout:

1. Pin an exact Servo git revision (or a released version whose API has been tested) and its matching Rust toolchain.
2. Build a standalone Rust proof-of-concept using Servo's documented minimal embedder example.
3. Verify offscreen rendering and RGBA readback on one development platform.
4. Add `rusted-web` as an optional workspace member / feature.
5. Build the C ABI and test it from a tiny C# harness before integrating SMAPI.
6. Add Windows x64 and Linux x64 packaging separately; validate Linux native and Proton/Wine independently.
7. Bundle the Servo resource files and required native libraries with the web-enabled package. Verify resource lookup and license notices.
8. Keep the default Rusted package free of Servo binaries unless web rendering is enabled.

Do not assume that the existing `build.sh` cross-compilation setup for `rusted-bridge` is sufficient for Servo. Servo's own build instructions and native dependencies must be followed for each target. Start with native builds; add cross-compilation only after a reproducible host build exists.

## 10. Security and permissions

A browser engine executes untrusted web content. Rusted is an in-game mod framework, so web rendering must be opt-in and constrained.

Minimum safeguards:

- Default to local packaged content; require explicit opt-in for remote navigation.
- Allowlist URL schemes (normally `https` and approved local asset URLs); reject `file://`, arbitrary custom schemes and unexpected redirects unless explicitly supported.
- Do not expose SMAPI, filesystem, game state, mod APIs or native bridge methods to page JavaScript by default.
- Do not automatically grant camera, microphone, geolocation, notifications, clipboard or file-picker permissions.
- Define a permission delegate that denies requests unless the host mod explicitly handles them.
- Restrict navigation and subresource requests according to the host mod's policy.
- Avoid embedding secrets or authentication tokens in page URLs.
- Provide a clear way to close a page and stop its network activity.
- Keep Servo and its dependencies updated; report security issues through the project's normal process.

A browser surface is not a trusted UI boundary. Treat page content and all messages from it as untrusted input.

## 11. Performance targets and profiling

Do not compare Servo's full browser pipeline directly with the current Taffy command renderer. They solve different problems.

Measure separately:

- Cold Servo startup time and warm WebView creation time.
- Resident memory and native binary size.
- HTML/CSS/JS page load and script execution time.
- Time from frame-ready notification to completed frame.
- Readback time, C# upload time and SpriteBatch draw time.
- CPU/GPU usage at 30, 60 and 120 Hz.
- Memory growth after repeated navigation and close/reopen cycles.

Start with a capped web surface resolution and update only when Servo reports a new frame. Avoid reading pixels every game frame if the page has not changed. Consider throttling hidden/inactive views.

## 12. Implementation milestones

| Milestone | Deliverable | Acceptance criteria |
|---|---|---|
| M0 | Standalone Servo spike | Pinned Servo build loads a local HTML page |
| M1 | Offscreen rendering | RGBA frame is captured and displayed in a minimal host |
| M2 | Rust C ABI | Create, navigate, resize, acquire frame and destroy work from a C harness |
| M3 | C# adapter | A static web page appears inside a Rusted/MonoGame menu |
| M4 | Input | Mouse, wheel, keyboard, focus and resize work |
| M5 | Packaging | Windows x64 and Linux x64 packages load bundled Servo resources |
| M6 | Hardening | Permission policy, navigation restrictions, crash handling and profiling complete |
| M7 | Optional GPU interop | Only pursued if measured CPU readback cost justifies the complexity |

**Definition of done:** a web page can be opened in a Rusted menu, rendered through an explicitly owned offscreen surface, interacted with, resized and closed without breaking the existing RustedUI path. The no-Servo build must continue to work unchanged.

## 13. References

- [Servo project](https://servo.org/)
- [Servo crate API (current docs)](https://doc.servo.org/servo/)
- [Servo WebView](https://doc.servo.org/servo/struct.WebView.html)
- [Servo RenderingContext](https://doc.servo.org/servo/trait.RenderingContext.html)
- [Servo embedding overview](https://book.servo.org/embedding/overview.html)
- [Servo minimal winit embedder](https://github.com/servo/servo/tree/main/ports/winit)
- [Servo repository and build instructions](https://github.com/servo/servo)
- [WebRender API](https://doc.servo.org/webrender/)

> **Important:** Servo's embedding API and build requirements evolve. Re-check the exact API docs, Servo Book and pinned source revision before turning the illustrative snippets in this document into production code.
