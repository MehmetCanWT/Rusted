# ⚡ RUSTED UI

### Next-Generation High-Performance In-Game UI & HUD Engine for Stardew Valley 1.6+ (SMAPI 4.0+)

**✦ ZERO GC ALLOCATION | ✦ RUST TAFFY ENGINE | ✦ 10 LIVE THEMES | ✦ STEAM DECK VERIFIED**

---

## 🌟 What is RustedUI?

**RustedUI** is an ultra-fast, zero-lag in-game user interface framework and visual showcase for **Stardew Valley 1.6+**.

Traditional UI mods in C# frequently suffer from garbage collection (GC) micro-stutters and frame spikes when drawing detailed menus or telemetry overlays. **RustedUI eliminates this completely**: by calculating all layout, styling, and text wrapping inside an embedded, compiled **Rust engine (powered by Taffy v0.7 Flexbox & Grid)** with arena memory and 0 bytes of C# garbage collection per frame, it outputs raw MonoGame `SpriteBatch` draw calls in microseconds (**~0.03–0.08 ms**).

Whether you are a **player** who wants clean, customizable HUDs and menus that match Stardew Valley's art style, or a **mod author** looking to build interfaces using HTML, Tailwind CSS, React/TSX, or C# Fluent Builders, RustedUI is built for you!

---

## ✨ Key Features

- **🦀 Zero Garbage Collection Overhead:** Smooth 60/120+ FPS gameplay with 0 bytes of managed C# allocations during rendering.
- **🎨 10 Instant In-Game Themes:**
  - **Pelican Classic:** Authentic vanilla Stardew Valley wooden and parchment 9-slice boxes.
  - **Shadcn Zinc Dark:** Sleek modern dark mode (`#09090B` / `#18181B`).
  - **Wizard Arcane:** Mystic deep violet and starlight gold.
  - **JojaCorp Blue:** Corporate industrial cobalt and cyan.
  - **Junimo Forest:** Organic emerald and dewdrop green.
  - **Catppuccin Mocha, Tokyo Night, Dracula, Nord & Cyberpunk Neon:** Highly requested community palettes!
- **💬 Theme-Aware Floating Tooltips:** Hovering over any HUD element or item slot shows an interactive tooltip styled to match your selected theme.
- **🔒 Per-Mod Theme Isolation:** Third-party mods keep their own custom branding without being overridden by global theme changes (unless explicitly opted in).
- **🎮 Steam Deck & Gamepad Verified:** Seamless controller support, full-screen UI scaling, and proton compatibility with no third-party overlay hooks.
- **📦 Built-In Widget Catalog:** Item slots with quality stars, dynamic friendship heart meters, gold counters, progress bars, text input with keyboard support, smooth sliders, and hardware-clipped scroll views.
- **📐 Modder-Friendly Workflows:** Build menus using **HTML + Tailwind CSS** (with live in-game hot-reload), **React/Vite TSX** ahead-of-time components, or a type-safe **GPUI-style C# Fluent Builder**.

---

## ⌨️ Controls & Shortcuts

| Keybind | Function | Details |
| :--- | :--- | :--- |
| **F8** | **RustedUI Control Panel** | Opens the main control panel, theme selector, audio toggles, and feature test lab. Can also be opened on the Title Screen! |
| **F7** | **Toggle Mini-HUD** | Quickly shows or hides the lightweight in-game telemetry & quick-action overlay (Disabled by default). |
| **ESC / Right-Click** | **Close Menu** | Closes active RustedUI menus and returns control to the game. |

---

## 📦 Installation

1. Ensure you have **SMAPI 4.0+** installed for Stardew Valley 1.6+.
2. Download the **RustedUI** zip file from this CurseForge page.
3. Unzip the archive and place the `RustedUI` folder into your `Stardew Valley/Mods` folder.
4. Launch the game through SMAPI. Press **F8** in-game or on the title screen to explore!
5. *(Optional)* If you have **Generic Mod Config Menu (GMCM)** installed, you can configure hotkeys and HUD visibility from the in-game settings menu.

---

## 🛡️ Security & Native Binary Transparency

RustedUI includes pre-compiled, statically-linked native libraries (`librusted_bridge.so` for Linux and `rusted_bridge.dll` for Windows / Steam Deck Proton).

> **Security Disclosure:**
> - The native library performs **strictly in-memory math layout calculations** (CSS Flexbox/Grid via the open-source Taffy engine).
> - It makes **zero network calls**, touches no disk/file systems, executes no OS shell processes, and collects zero telemetry.
> - It is compiled with static CRT (`+crt-static`), requiring no external Visual C++ runtime installs.
> - Full source code is publicly audited and available on [GitHub](https://github.com/MehmetCanWT/Rusted).

---

## 📄 License & Permissions

RustedUI is licensed under the **RustedUI Source-Available Software License (v1.0)**:

- **Free for Players:** 100% free to download, install, and enjoy.
- **Free for Mod Authors:** You are fully authorized and encouraged to use RustedUI as a library or dependency for your own Stardew Valley mods without any payment or royalties.
- **Redistribution Restricted:** Direct re-hosting, mirror uploading, or commercial redistribution of RustedUI on third-party sites or paywalled platforms is strictly prohibited and requires an authorized commercial distribution agreement.

---

*Crafted with ❤️ for the Stardew Valley Modding Community.*
