# RustedUI — Quickstart, Keybinds, Themes & Player Guide

**RustedUI (`Mehmetcan.RustedUI`)** is a high-performance, zero-GC in-game UI and HUD framework for **Stardew Valley 1.6+ (SMAPI 4.1+)** powered by a native **Rust + Taffy v0.7** layout engine and **MonoGame `SpriteBatch`** hardware rendering.

---

## 1. Installation & Building from Source

### Prerequisites
- **Stardew Valley 1.6+** with **SMAPI 4.1+** installed
- **Rust Toolchain** (`rustup`, `cargo`, and `cargo-xwin` for Windows/Proton cross-compilation)
- **.NET 6.0 SDK** (`dotnet`)
- **Node.js 18+** (`node` for compiling React/TSX templates)

### One-Command Build & Deploy
Run the root build script to compile TSX components, run all Rust unit tests, build both Linux (`librusted_bridge.so`) and Windows/Proton (`rusted_bridge.dll` with `+crt-static`) native binaries, and deploy the mod directly to your Stardew Valley `Mods/Rusted.Smapi` folder:

```bash
./build.sh
```

---

## 2. Default Keybinds & In-Game Controls

| Keybind | Action | Description |
|---|---|---|
| **`F8`** | **Open / Close Control Panel** | Opens the main RustedUI Control Panel from anywhere (Title Screen or In-Game). |
| **`F7`** | **Toggle In-Game Mini-HUD** | Quickly shows or hides the top-left RustedUI telemetry & quick-access HUD overlay. |
| **`ESC`** | **Close Active Menu** | Closes any open RustedUI menu with vanilla Stardew sound effects. |

---

## 3. Optional Generic Mod Config Menu (GMCM) Integration

RustedUI includes built-in, **100% optional** support for **Generic Mod Config Menu (`spacechase0.GenericModConfigMenu`)**:
- **Not Required**: RustedUI has **zero hard dependencies**. If GMCM is not installed, RustedUI works out-of-the-box using its own in-game Control Panel (`F8`) and `config.json`.
- **If GMCM Is Installed**: RustedUI automatically registers a configuration page on the Stardew Valley title screen and in-game options menu allowing players to customize:
  - **Show In-Game Mini-HUD** (`true` / `false`)
  - **Toggle Mini-HUD Keybind** (Default: `F7`)
  - **Open RustedUI Menu Keybind** (Default: `F8`)
  - **Default UI Color Theme** (10 themes)
  - **Enable UI Sound Effects** (`true` / `false`)

### `config.json` Schema
```json
{
  "ShowHud": true,
  "EnableUiSounds": true,
  "OpenMenuKey": "F8",
  "ToggleHudKey": "F7",
  "DefaultTheme": "PelicanClassic"
}
```

---

## 4. Built-In 10 Live Color Themes

When you select a theme in RustedUI, **every part of RustedUI** — including windows, cards, progress bars, sliders, buttons, badges, HUD overlays, and **floating hover tooltips** — immediately switches to that theme:

| Theme ID | Config Name | Style & Palette |
|---|---|---|
| `0` | `PelicanClassic` | Authentic Stardew Valley 9-Slice wooden borders & warm parchment (`#F5D597` / `#56160C`) |
| `1` | `ShadcnZincDark` | Modern high-contrast Zinc Dark UI (`#09090B` / `#18181B` / Emerald `#10B981`) |
| `2` | `WizardArcane` | Rasmodius Astral Void & Starlight Gold (`#0F081D` / `#A855F7` / `#FFD700`) |
| `3` | `JojaCorp` | Industrial Corporate Slate & Cyber Blue (`#0F172A` / `#0284C7` / `#38BDF8`) |
| `4` | `JunimoForest` | Organic Deep Moss & Dewdrop Cream (`#14261C` / `#86EFAC` / `#FDE047`) |
| `5` | `CatppuccinMocha` | Soothing Pastel Dark Mocha (`#11111B` / `#1E1E2E` / Mauve `#CBA6F7`) |
| `6` | `TokyoNight` | Neon-Lit Downtown Tokyo Night (`#16161E` / `#1A1B26` / `#7AA2F7`) |
| `7` | `Dracula` | Iconic Vampire Dark (`#1E1F29` / `#282A36` / Pink `#FF79C6` & Purple `#BD93F9`) |
| `8` | `Nord` | Arctic Frost Bluish-Grey Polar (`#242933` / `#2E3440` / Frost `#88C0D0`) |
| `9` | `Cyberpunk` | High-Voltage Night City Neon (`#0A0616` / `#FF007F` / `#00F0FF` / `#FCEE0A`) |

---

## 5. Navigating the Built-In Menus & All-Features Test Lab

1. **Mini-HUD Overlay (Top-Left)**:
   - Displays live render frame time (`0.04ms`), a synchronized progress bar, and a **Control Panel** button.
   - Hovering over the HUD card or button displays a **theme-aware floating tooltip**.
2. **Main Control Panel (`F8`)**:
   - **Theme Bar**: Switch between all 10 themes in real time and view live **Mod CPU %** and **Rust UI RAM (KB)** usage.
   - **1. Telemetry & HUD**: View render time (`ms`), draw command count, event counter, and toggle the Mini-HUD or UI sound effects using Shadcn `<switch>` controls.
   - **2. Menu Navigation**: Open the **Credits & Architecture** screen, jump straight to the **All-Features Test Lab**, or test game state integration (`+500G Add Gold`).
3. **Credits & Architecture Menu**:
   - Displays the RustedUI core stack badges, architecture summary, and a 10-heart rating widget.
4. **All-Features Test Lab**:
   - **Top Mode & Tab Bar**:
     - **`Split` Tab**: Shows both `1. Interactive Controls` and `2. Slots, Icons & Scroll` side-by-side.
     - **`Inputs` Tab**: Expands `1. Interactive Controls` to full width (`flex-1`) and hides the Inventory panel.
     - **`Inventory` Tab**: Expands `2. Slots, Icons & Scroll` to full width (`flex-1`) and hides the Inputs panel.
     - **`C# Mode` / `TSX Mode` Button**: Live-switches the entire menu between the compiled TSX/HTML template and the programmatic GPUI C# Fluent Builder.
   - **Interactive Widgets**: Continuous drag `<slider>`, `<progress-bar>`, keyboard `<text-input>` + `<trash-button>`, `<heart-rating>` with left/right arrows, Shadcn `<switch>`, vanilla `<checkbox>`, 5 `<item-slot>` widgets with quality stars and stack counts, 6 Lucide pixel-art icons, Shadcn `<badge>` variants, and a hardware-clipped `<scroll-view>`.
