# ⚡ RUSTED UI

### Next-Generation High-Performance In-Game UI & HUD Framework for Stardew Valley 1.6+ (SMAPI 4.0+)

**✦ ZERO GC ALLOCATION | ✦ RUST TAFFY ENGINE | ✦ 10 LIVE THEMES | ✦ STEAM DECK READY**

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-black?style=for-the-badge&logo=github)](https://github.com/MehmetCanWT/Rusted)
[![Requires SMAPI 4.0+](https://img.shields.io/badge/Requires-SMAPI%204.0+-brightgreen?style=for-the-badge)](https://www.nexusmods.com/stardewvalley/mods/2400)
[![Stardew Valley 1.6+](https://img.shields.io/badge/Stardew%20Valley-1.6+-orange?style=for-the-badge)](https://stardewvalley.net)
[![License: Source-Available](https://img.shields.io/badge/License-Source--Available-blue?style=for-the-badge)](https://github.com/MehmetCanWT/Rusted/blob/main/LICENSE)

---

## 🌟 What is RustedUI?

**RustedUI** is an ultra-fast, zero-lag in-game user interface framework and visual showcase for **Stardew Valley 1.6+ (SMAPI 4.0+)**.

Traditional UI mods in C# frequently suffer from garbage collection (GC) micro-stutters and frame spikes when drawing detailed menus or HUD overlays. **RustedUI eliminates this completely**: by computing all layout, styling, and text-wrapping inside a native **Rust engine (powered by Taffy v0.7 Flexbox & Grid)** with arena memory and 0 bytes of C# garbage collection per frame, it outputs raw MonoGame `SpriteBatch` draw commands in microseconds (**~0.03–0.08 ms**).

Whether you are a **player** who wants beautiful, customizable HUDs and menus that feel like native Stardew Valley, or a **mod author** looking to build cutting-edge menus using HTML, Tailwind CSS, React/TSX, or C# Fluent Builders, RustedUI is built for you!

---

## ✨ Key Features at a Glance

- **🦀 Zero Garbage Collection Overhead:** Pure 60/120+ FPS gameplay with 0 bytes of managed C# allocations per frame during rendering.
- **🎨 10 Live Color Themes:**
  - **Pelican Classic:** Authentic vanilla Stardew wooden and parchment 9-slice boxes.
  - **Shadcn Zinc Dark:** Ultra-clean modern dark mode (`#09090B` / `#18181B`).
  - **Wizard Arcane:** Mystic deep violet and starlight gold.
  - **JojaCorp Blue:** Industrial corporate cobalt and cyan.
  - **Junimo Forest:** Organic emerald and dewdrop green.
  - **Catppuccin Mocha, Tokyo Night, Dracula, Nord & Cyberpunk Neon:** Vibrant aesthetic palettes!
- **💬 Theme-Aware Floating Tooltips:** Hovering over any HUD element or item slot shows an interactive tooltip styled to match your selected theme.
- **🔒 Per-Mod Theme Isolation:** Third-party mods keep their own custom visual branding without being overridden by global theme changes (unless explicitly opted in).
- **🎮 Steam Deck & Gamepad Ready:** Native controller support, full-screen UI scaling, and proton compatibility with no third-party overlay hooks.
- **📦 Rich Built-In Widget Catalog:** Inventory item slots with quality stars, friendship heart meters, coin counters, progress bars, text input with keyboard support, smooth sliders, and hardware-clipped scroll views.
- **📐 Ergonomic Modder Tooling:** Build menus with **HTML + Tailwind CSS** (with live in-game hot-reload), **React/Vite TSX** ahead-of-time components, or a type-safe **GPUI-style C# Fluent Builder**.

---

## ⌨️ Default Keybinds

| Keybind | Action | Description |
| :--- | :--- | :--- |
| **F8** | **Open Control Panel** | Opens the RustedUI Control Panel, Theme Switcher, Audio Toggles, and Feature Test Lab. Can also be opened on the Title Screen! |
| **F7** | **Toggle Mini-HUD** | Quickly shows or hides the in-game telemetry & quick-action overlay (Disabled by default). |
| **ESC / Right-Click** | **Close Menu** | Closes active RustedUI menus and returns control to the game. |

---

## 📦 Installation

1. Install the latest version of [SMAPI (4.0.0 or higher)](https://www.nexusmods.com/stardewvalley/mods/2400).
2. Download the **RustedUI** zip file using **Vortex** or **Manual Download**.
3. If installing manually, extract the archive into your `Stardew Valley/Mods/` directory.
4. Launch the game using SMAPI. Press **F8** in-game or on the title screen to verify installation!
5. *(Optional)* If you have **Generic Mod Config Menu (GMCM)** installed, you can configure hotkeys and HUD visibility from the in-game settings menu.

---

## 🛡️ Security & Native Binaries Disclosure

RustedUI includes pre-compiled native libraries (`librusted_bridge.so` for Linux and `rusted_bridge.dll` for Windows / Steam Deck Proton).

> **Security Guarantees:**
> - **Pure CPU Layout Math:** The native library performs strictly in-memory layout computations (Taffy Flexbox/Grid).
> - **Zero Network / Zero OS Access:** It makes 0 network calls, touches no disk/file systems, and collects 0 telemetry.
> - **No VC++ Redistributable Needed:** Compiled with static CRT (`+crt-static`) for 100% out-of-the-box Proton / Wine compatibility.
> - **Public Source Code:** Fully open for audit on [GitHub](https://github.com/MehmetCanWT/Rusted).

---

## 📄 License & Permissions

RustedUI is distributed under the **RustedUI Source-Available Software License (v1.0)**:

- **Free for Players:** 100% free to download, install, and enjoy.
- **Free for Mod Authors:** You are free to use RustedUI as a library or dependency in your own Stardew Valley mods without paying any royalties or fees.
- **Redistribution Restricted:** Re-hosting, mirror uploading, or commercial redistribution on third-party sites or paywalled services is strictly prohibited without an explicit commercial license agreement.

---

*Crafted with ❤️ for the Stardew Valley Modding Community.*
