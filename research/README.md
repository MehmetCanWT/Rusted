# RustedUI — Architectural & Engineering Research Reports (`01` – `19`)

This directory contains the 19 deep technical specifications and architectural research reports produced for **Rusted (RustedUI)**:

| # | Report File | Topic |
|---|---|---|
| **01** | [01-project-baseline-and-architecture-plan.md](./01-project-baseline-and-architecture-plan.md) | Project Baseline, Hybrid Rust + C# Architecture & Master Plan |
| **02** | [02-smapi-monogame-rendering-and-vanilla-assets.md](./02-smapi-monogame-rendering-and-vanilla-assets.md) | SMAPI 4.x Lifecycle, MonoGame `SpriteBatch`, Vanilla 9-Slice Coordinates & Audio Cues |
| **03** | [03-rust-ui-engine-taffy-and-styling-pipeline.md](./03-rust-ui-engine-taffy-and-styling-pipeline.md) | Rust `SlotMap` + `Taffy v0.7` Layout Engine, Tailwind CSS Parser & GPUI Builder |
| **04** | [04-csharp-rust-translation-layer-and-crossplatform-abi.md](./04-csharp-rust-translation-layer-and-crossplatform-abi.md) | Zero-GC Blittable C-ABI FFI Protocol, `[SuppressGCTransition]` & Proton `+crt-static` |
| **05** | [05-monogame-scissor-mouse-and-coordinate-audit.md](./05-monogame-scissor-mouse-and-coordinate-audit.md) | MonoGame Hardware `ScissorRectangle`, Mouse Cursor Drawing & UI Coordinate Spaces |
| **06** | [06-taffy-flexbox-wrapping-and-sizing-audit.md](./06-taffy-flexbox-wrapping-and-sizing-audit.md) | Taffy v0.7 Flexbox Shrink/Min-Size Rules, Multi-Line Text Wrapping & Scroll Clipping |
| **07** | [07-font-metrics-baseline-and-component-geometry.md](./07-font-metrics-baseline-and-component-geometry.md) | Stardew `SpriteFont` Glyph Metrics, Baseline Alignment & Component Geometry |
| **08** | [08-tailwind-shadcn-color-themes-and-lucide-icons.md](./08-tailwind-shadcn-color-themes-and-lucide-icons.md) | Shadcn/UI Design Tokens, Theme System & Lucide 16x16 Pixel-Art Icon Atlas |
| **09** | [09-react-vite-tsx-component-authoring-architecture.md](./09-react-vite-tsx-component-authoring-architecture.md) | React / Vite TypeScript (`.tsx`) AOT Component Compiler & Hot-Reload Architecture |
| **10** | [10-popular-web-and-shadcn-theme-catalog.md](./10-popular-web-and-shadcn-theme-catalog.md) | 25+ Popular Internet, Terminal, Shadcn & Stardew Seasonal Themes + Per-Mod Theme Isolation |
| **11** | [11-custom-image-sprite-and-asset-pipeline.md](./11-custom-image-sprite-and-asset-pipeline.md) | Custom `<img src="..." />`, SpriteSheets, Portraits, `ItemRegistry` & Texture Atlas Pipeline |
| **12** | [12-responsive-viewport-scaling-and-window-resizing.md](./12-responsive-viewport-scaling-and-window-resizing.md) | Responsive Viewport Scaling (`vw`/`vh`, `sm:`/`md:`/`lg:`), `uiScale` & Window Resizing |
| **13** | [13-custom-fonts-bmfont-ttf-and-unicode-i18n.md](./13-custom-fonts-bmfont-ttf-and-unicode-i18n.md) | Custom TTF/OTF Rasterization (`fontdue`), Dynamic Glyph Atlas, `SpriteText` & Unicode i18n |
| **14** | [14-vite-react-tsx-tailwind-and-shadcn-ecosystem.md](./14-vite-react-tsx-tailwind-and-shadcn-ecosystem.md) | Full Shadcn/UI `.tsx` Component Library & Dual Browser/In-Game Live Preview Workflow |
| **15** | [15-rust-csharp-linting-fast-builds-and-binary-size.md](./15-rust-csharp-linting-fast-builds-and-binary-size.md) | Clippy + Roslyn + Biome Linting, Fast Builds, `<280 KB` Binary Size & Live CPU/RAM Telemetry |
| **16** | [16-smapi-mod-ecosystem-and-public-sdk-api.md](./16-smapi-mod-ecosystem-and-public-sdk-api.md) | SMAPI Public Modder SDK (`IRustedUiApi`), Pintail Proxy Rules & Per-Mod Theme Isolation |
| **17** | [17-gamepad-controller-navigation-and-accessibility.md](./17-gamepad-controller-navigation-and-accessibility.md) | Gamepad / Steam Deck 2D Spatial Navigation, Controller Mapping & Keyboard Accessibility |
| **18** | [18-animation-spring-physics-and-shader-effects.md](./18-animation-spring-physics-and-shader-effects.md) | 60/120 FPS Zero-Allocation Spring Physics Animations, Transitions & HLSL Shader Effects |
| **19** | [19-advanced-widgets-data-grids-and-drag-drop.md](./19-advanced-widgets-data-grids-and-drag-drop.md) | Virtualized Lists (`10,000+` rows), Drag-and-Drop Item Grids, Dropdowns, Modals & Keybind Recorders |
