# ⚡ RUSTED UI

### Next-Generation High-Performance In-Game UI & HUD Framework for Stardew Valley 1.6+ (SMAPI 4.0+)

**✦ ZERO GC ALLOCATION | ✦ RUST TAFFY ENGINE | ✦ 10 LIVE THEMES | ✦ STEAM DECK READY**

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-black?style=for-the-badge&logo=github)](https://github.com/MehmetCanWT/Rusted)
[![Requires SMAPI 4.0+](https://img.shields.io/badge/Requires-SMAPI%204.0+-brightgreen?style=for-the-badge)](https://www.nexusmods.com/stardewvalley/mods/2400)
[![Stardew Valley 1.6+](https://img.shields.io/badge/Stardew%20Valley-1.6+-orange?style=for-the-badge)](https://stardewvalley.net)
[![License: Source-Available](https://img.shields.io/badge/License-Source--Available-blue?style=for-the-badge)](https://github.com/MehmetCanWT/Rusted/blob/main/LICENSE)

---

## Description

**RustedUI** is an ultra-fast, zero-lag in-game user interface framework, HUD overlay, and visual showcase built specifically for **Stardew Valley 1.6+ (SMAPI 4.0+)**.

In heavily modded Stardew Valley setups, traditional C# UI mods frequently cause garbage collection (GC) micro-stutters and sudden FPS drops when rendering extensive menus or floating HUD elements. **RustedUI completely solves this problem**:
- It delegates all layout math, styling, and text wrapping to an embedded native **Rust engine powered by Taffy v0.7** (CSS Flexbox & Grid).
- Running with arena-allocated memory and **0 bytes of managed C# garbage collection per frame**, RustedUI synthesizes and passes flat command buffers directly to MonoGame's hardware `SpriteBatch` in microseconds (**~0.03–0.08 ms**).

Whether you are a **player** looking for a polished, themeable telemetry overlay and responsive menus, or a **mod author** seeking to build custom interfaces using HTML + Tailwind CSS, React/TSX, or a fluent C# builder, RustedUI provides the modern foundation you need.

---

## Installation instructions

Follow these simple steps to install RustedUI:

1. **Prerequisites:** Make sure you have installed the latest version of [SMAPI](https://www.nexusmods.com/stardewvalley/mods/2400) (version 4.0.0 or higher) for Stardew Valley 1.6+.
2. **Download:** Download the **RustedUI** archive using your favorite mod manager (**Vortex**) or choose **Manual Download**.
3. **Extract:** If installing manually, extract the downloaded zip file into your `Stardew Valley/Mods/` directory so you have a `Mods/RustedUI/` folder containing `manifest.json`.
4. **Launch & Verify:** Launch the game through SMAPI. Press **F8** anywhere (in-game or on the Title Screen) to open the RustedUI Control Panel!
5. **Keybinds:**
   - **F8:** Open / close the RustedUI Control Panel and Feature Test Lab.
   - **F7:** Toggle the in-game Mini-HUD overlay on / off (Disabled by default).
   - **ESC / Right-Click:** Close active RustedUI menus.

---

## Main features

- **🦀 Zero Garbage Collection Overhead:** 0 bytes of managed C# heap allocations per frame while rendering. Keeps your frame pacing rock-solid at 60 / 120+ FPS.
- **🎨 10 Instant In-Game Themes:** Switch instantaneously between 10 built-in palettes:
  - **Pelican Classic:** Authentic vanilla Stardew wooden and parchment 9-slice boxes.
  - **Shadcn Zinc Dark:** Minimalist modern high-contrast dark mode (`#09090B` / `#18181B`).
  - **Wizard Arcane:** Mystic deep violet and starlight gold.
  - **JojaCorp Blue:** Industrial corporate cobalt and electric cyan.
  - **Junimo Forest:** Organic emerald and dewdrop green.
  - **Catppuccin Mocha, Tokyo Night, Dracula, Nord & Cyberpunk Neon:** Popular aesthetic themes!
- **💬 Theme-Aware Floating Tooltips:** Hovering over any HUD element or item slot renders an interactive tooltip styled with the active theme tokens.
- **🔒 Per-Mod Theme Isolation:** Third-party mods retain their unique aesthetic without being forcefully overridden by global theme changes.
- **🎮 Gamepad & Steam Deck Ready:** Full controller support with snap navigation and out-of-the-box Proton / Linux compatibility.
- **📦 Rich Built-In Widget Catalog:** Ready-to-use widgets including inventory slots with item quality stars, friendship heart meters, coin counters, text inputs with keyboard typing, smooth sliders, switches, checkboxes, and hardware-clipped scroll views.
- **📐 3 Flexible Authoring Workflows for Modders:**
  1. **HTML + Tailwind CSS:** Author markup with hot-reload support (`assets/ui/`).
  2. **React / Vite TSX:** Ahead-of-time component compilation (`ui-tsx/`).
  3. **C# Fluent API:** Construct declarative trees programmatically in pure C# with `RustedContext`.

---

## Requirements

RustedUI is lightweight and self-contained:

- **Required:**
  - **Stardew Valley 1.6+** (Steam, GOG, or PC Game Pass).
  - [SMAPI - Stardew Modding API](https://www.nexusmods.com/stardewvalley/mods/2400) (Version **4.0.0 or higher**).
- **Optional:**
  - [Generic Mod Config Menu (GMCM)](https://www.nexusmods.com/stardewvalley/mods/5098): If installed, allows you to configure HUD toggles, keybinds, and audio cues directly in Stardew Valley's in-game options.
- **No External Runtimes Needed:**
  - RustedUI includes self-contained native libraries (`librusted_bridge.so` for Linux/Deck and `rusted_bridge.dll` statically linked with `+crt-static` for Windows/Proton). No Visual C++ redistributable or Rust compiler installation is required.

---

## Shout outs

Special thanks and appreciation to the incredible creators and open-source projects that made RustedUI possible:

- **ConcernedApe (Eric Barone):** For creating Stardew Valley and giving the modding community endless inspiration and support with the 1.6 update.
- **Pathoschild:** For SMAPI, `SMAPI.ModBuildConfig`, and countless years of dedicated leadership in Stardew Valley modding infrastructure.
- **The Dioxus Labs Team:** For [Taffy](https://github.com/DioxusLabs/taffy), the blazing-fast Rust implementation of CSS Flexbox and CSS Grid.
- **spacechase0:** For Generic Mod Config Menu (GMCM), the gold standard for in-game configuration.
- **The Rust & MonoGame Communities:** For providing unmatched performance tooling and robust game engine foundations.

---

## License & Permissions

RustedUI is distributed under the **RustedUI Source-Available Software License (v1.0)**:
- **Free for Players:** 100% free to download, install, and enjoy.
- **Free for Mod Authors:** You are fully authorized and encouraged to use RustedUI as a library or dependency for your own Stardew Valley mods without any royalties or fees.
- **Redistribution Restricted:** Direct re-hosting, mirror uploading, or commercial redistribution on third-party sites or paywalled services is strictly prohibited without an explicit commercial license agreement.

---

*Crafted with ❤️ for the Stardew Valley Modding Community.*
