# ⚡ Rusted (RustedUI)

**A Blazing-Fast, Zero-GC, Rust-Powered UI & HUD Framework for Stardew Valley 1.6+ (SMAPI 4.1+)**

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](#)
[![Taffy](https://img.shields.io/badge/Layout-Taffy_v0.7_Flexbox_%26_Grid-blue)](#)
[![SMAPI](https://img.shields.io/badge/Stardew_Valley_1.6+-SMAPI_4.1+-green)](#)
[![Shadcn/UI](https://img.shields.io/badge/Design-Shadcn%2FUI_%2B_Tailwind_%2B_Lucide-purple)](#)
[![License](https://img.shields.io/badge/License-Source--Available%20(Free%20for%20Modders)-blue.svg)](LICENSE)

**RustedUI (`Mehmetcan.RustedUI`)** brings modern web and desktop UI engineering (**Rust `SlotMap` + `Taffy v0.7` Flexbox/Grid**, **React/Vite `.tsx`**, **HTML + Tailwind CSS**, **Shadcn/UI**, **Lucide 16x16 Pixel-Art Icons**, and a **GPUI-inspired C# Fluent API**) into **Stardew Valley 1.6+**, rendering directly through MonoGame's hardware `SpriteBatch` with **0 Bytes of C# Garbage Collection per frame** and **~0.03–0.08 ms** layout/render time.

---

## 📥 Download & Installation

> [!TIP]
> **For Players & Modding**: Pre-compiled, ready-to-play mod packages are distributed exclusively on official mod platforms:
> - **[Nexus Mods (Official Page)](#)**
> - **[CurseForge (Official Page)](#)**
>
> **Installation Steps:**
> 1. Download the latest release `.zip` from Nexus Mods or CurseForge.
> 2. Extract the archive into your `Stardew Valley/Mods/` folder (requires SMAPI 4.0+).
> 3. Launch the game via SMAPI. Press **F8** in-game or on the title screen to open the RustedUI Control Panel.
>
> *(The GitHub repository hosts the source code for building, contributing, and developing dependent mods).*

---

## ✨ Key Features

- **🦀 Zero-GC Native Rust UI Engine (`crates/rusted-core`)**:
  - Generational `SlotMap` DOM arena paired with `Taffy v0.7` Flexbox & CSS Grid layout.
  - Sub-millisecond layout and command synthesis (`~0.04 ms` per frame) shared zero-copy with C# via blittable `RenderSliceView` and `EventSliceView` spans (`[SuppressGCTransition]`).
- **🎨 10 Live Switchable Themes + Per-Mod Theme Isolation**:
  - Switch instantaneously between **Pelican Classic** (authentic vanilla 9-slice wooden/parchment boxes), **Shadcn Zinc Dark**, **Wizard Arcane**, **JojaCorp Blue**, **Junimo Forest**, **Catppuccin Mocha**, **Tokyo Night**, **Dracula**, **Nord Polar**, and **Cyberpunk Neon**.
  - **Theme-Aware Floating Tooltips**: Hovering over any HUD element or menu widget renders a crisp floating tooltip styled with the active theme's tokens.
  - **Per-Mod Theme Isolation**: Changing RustedUI's theme applies to RustedUI's own HUD and menus while **preserving third-party mods' custom themes**, unless the external mod explicitly opts in (`UseRustedUiTheme = true` or `use-rusted-theme="true"`).
- **🧩 3 Ergonomic Authoring Layers**:
  1. **React / Vite JSX/TSX Component Compiler (`ui-tsx/`)**: Write type-safe `.tsx` components with Shadcn primitives (`<Badge>`, `<Switch>`, `<Icon>`, `<ThemeBar>`) that compile ahead-of-time into RustedUI templates.
  2. **HTML / XML + Tailwind CSS with Live `<20 ms` Hot-Reload**: Edit `.html` files in `assets/ui/` while the game is running and watch menus update live without losing widget state.
  3. **GPUI-Inspired C# Fluent Builder (`RustedContext`)**: Build trees programmatically in pure C# (`ui.Div("flex flex-row gap-4").Children(...)`).
- **🛠️ Rich Built-In Widget Catalog**:
  - `<banner>`, `<panel>`, `<card>`, `<tab>`, `<button>`, `<close-button>`, `<trash-button>`, `<arrow-left/right/up/down>`
  - `<slider>` (continuous mouse drag), `<progress-bar>`, `<switch>` (Shadcn toggle), `<checkbox>` (vanilla checkbox)
  - `<text-input>` (live keyboard typing + Backspace + clear)
  - `<item-slot>` (`64x64` inventory slot with object sprite, silver/gold/iridium quality stars, and stack counter)
  - `<heart-rating>` (dynamic `1..14` friendship hearts), `<coin-icon>`
  - `<icon>` (procedural `16x16` crisp pixel-art **Lucide Icon Atlas** generated directly in GPU memory)
  - `<scroll-view>` (hardware `GraphicsDevice.ScissorRectangle` clipping with mouse wheel & arrow scrolling)
- **⚙️ Optional Generic Mod Config Menu (GMCM) Support**:
  - Configure HUD visibility, keybinds (`F8` Control Panel, `F7` Toggle Mini-HUD), default theme, and UI sounds either via GMCM (if installed) or directly inside RustedUI's Control Panel (`F8`) — with **zero hard dependencies**.
- **🌍 Cross-Platform Native Binary ABI**:
  - Works out-of-the-box on **Linux native** (`librusted_bridge.so`), **Windows x64** (`rusted_bridge.dll`), and **Linux Proton / Wine / Steam Deck** (compiled via `cargo-xwin` with static CRT `+crt-static` so `vcruntime140.dll` is never required).

---

## 🏗️ Repository Structure

```text
Rusted/
├── Cargo.toml                     # Rust Workspace (rusted-core + rusted-bridge)
├── build.sh                       # One-command TSX -> Rust -> Cross-Win64 -> C# SMAPI build & deploy
├── crates/
│   ├── rusted-core/               # Core Rust UI Engine (SlotMap Arena, Taffy v0.7, Tailwind, 10 Themes, XML Parser)
│   └── rusted-bridge/             # C-ABI cdylib exporting zero-copy FFI functions (.so / .dll)
├── ui-tsx/                        # React / Vite JSX/TSX Component Compiler & Shadcn/Lucide Primitives
├── src/
│   └── Rusted.Smapi/              # C# SMAPI Mod, MonoGame SpriteBatch Renderer, Fluent Builder & GMCM Integration
├── docs/                          # Comprehensive Player & Modder Usage Guides (01 - 05)
└── research/                      # 19 Deep Engineering & Architectural Research Reports (01 - 19)
```

---

## 🚀 Quickstart (Build & Run)

```bash
# Compiles TSX templates, runs Rust unit tests, builds Linux .so + Windows .dll, and deploys to SMAPI Mods/
./build.sh
```

- Press **`F8`** in-game (or on the Title Screen) to open the **RustedUI Control Panel**.
- Press **`F7`** in-game to toggle the top-left **Mini-HUD Overlay**.
- Inside the Control Panel, click **Open All-Features Test Lab ->** (or go via **Credits & About ->**) to test all 10 live themes, `Split` / `Inputs` / `Inventory` tabs, live **CPU % & Rust UI RAM (KB)** telemetry, and TSX vs. C# Fluent modes.

---

## 📖 Documentation & Usage Guides (`docs/`)

1. **[01 — Quickstart, Keybinds, Themes & Player Guide](./docs/01-QUICKSTART-AND-PLAYER-GUIDE.md)**
2. **[02 — HTML / XML & Tailwind CSS Authoring Guide](./docs/02-HTML-AND-TAILWIND-GUIDE.md)**
3. **[03 — React / Vite TypeScript (`.tsx`) & Shadcn/UI Guide](./docs/03-REACT-VITE-TSX-GUIDE.md)**
4. **[04 — GPUI C# Fluent API, Modder SDK & Per-Mod Theme Isolation](./docs/04-CSHARP-FLUENT-API-AND-MOD-SDK.md)**
5. **[05 — Architecture, Zero-GC FFI Protocol & Cross-Platform ABI](./docs/05-ARCHITECTURE-AND-FFI-PROTOCOL.md)**
6. **[Research Index — 19 Deep Technical Specifications (`research/01` – `19`)](./research/README.md)**

---

## 💻 Authoring Examples

### 1. HTML + Tailwind CSS + Shadcn/Lucide (`assets/ui/*.html`)
```xml
<div id="hud_card" class="flex flex-col w-360 h-auto sdv-box p-4 gap-2"
     tooltip="RustedUI Zero-GC HUD Overlay (Rendered via Rust + MonoGame)">
    <div class="flex flex-row justify-between items-center w-full gap-2">
        <div class="flex flex-row items-center gap-2 shrink-0">
            <icon name="zap" class="lucide-icon lucide-md" />
            <span class="font-dialogue text-sdv-brown shadow">RustedUI</span>
        </div>
        <badge id="hud_perf" variant="secondary" class="sdv-badge badge-secondary">0.04ms</badge>
    </div>
    <progress-bar id="hud_bar" value="80" max="100" class="w-full h-5" />
    <button id="btn_open_menu" class="sdv-btn w-full" on:click="open_menu">Control Panel</button>
</div>
```

### 2. GPUI-Inspired C# Fluent Builder (`RustedContext`)
```csharp
_view.BuildWithFluent(ui =>
    ui.Div("flex flex-col w-640 h-auto sdv-box gap-3").Children(
        ui.Banner("RustedUI - GPUI C# Lab", "font-dialogue text-sdv-brown"),
        ui.Panel("flex flex-col gap-2 w-full").Children(
            ui.Text("Draggable Slider & Progress Bar:"),
            ui.Slider("slider_demo", 80f, 100f, "slider_changed", "w-full"),
            ui.ProgressBar("bar_demo", 80f, 100f, "w-full h-6"),
            ui.Button("btn_gold", "+500G", "give_gold")
        )
    )
);
```

---

## 📄 License & Permissions

RustedUI is licensed under the **RustedUI Source-Available Software License (v1.0)**:
- **Free for Players**: Free to download, install, and use in your game.
- **Free for Modders & Developers**: Free to use RustedUI as a library, framework, or dependency in your own Stardew Valley mods without any royalties or payments.
- **Redistribution & Mirroring Restricted**: Re-hosting, mirror-uploading, republishing, or redistributing the standalone mod or binaries on third-party sites or commercial platforms requires an explicit written commercial license and paid royalty agreement with the author. See [LICENSE](LICENSE) for full details.
