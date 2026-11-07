# Nexus Mods Description Template for RustedUI

> **How to Use in Nexus Mods:**
> 1. Open your mod page on Nexus Mods and click **Edit Mod**.
> 2. Click the **`[<>]` (Source Code / BBCode)** toggle button above the description box.
> 3. Paste the BBCode block below.
> 4. *(Note: This description contains NO mentions of CurseForge).*

```bbcode
[center]
[size=7][b][color=#F5D597]RUSTED[/color] [color=#E69138]UI[/color][/b][/size]
[size=4][i][color=#DCDCDC]Next-Generation High-Performance In-Game UI & HUD Framework for Stardew Valley 1.6+[/color][/i][/size]

[size=3][b][color=#10B981]✦ ZERO GC ALLOCATION[/color] | [color=#60A5FA]✦ RUST TAFFY ENGINE[/color] | [color=#F59E0B]✦ 10 LIVE THEMES[/color] | [color=#A78BFA]✦ STEAM DECK READY[/color][/b][/size]

[line]
[url=https://github.com/MehmetCanWT/Rusted][img]https://img.shields.io/badge/GitHub-Repository-black?style=for-the-badge&logo=github[/img][/url] [url=https://www.nexusmods.com/stardewvalley/mods/2400][img]https://img.shields.io/badge/Requires-SMAPI%204.0+-brightgreen?style=for-the-badge[/img][/url] [img]https://img.shields.io/badge/Stardew%20Valley-1.6+-orange?style=for-the-badge[/img] [img]https://img.shields.io/badge/License-Source--Available-blue?style=for-the-badge[/img]
[line]
[/center]

[size=5][b][color=#E69138]WHAT IS RUSTED?[/color][/b][/size]
[b]RustedUI[/b] is an ultra-fast, zero-lag in-game user interface framework and visual showcase for [b]Stardew Valley 1.6+ (SMAPI 4.0+)[/b]. 

Traditional UI mods in C# frequently suffer from garbage collection (GC) micro-stutters and frame spikes when drawing detailed menus or HUD overlays. [b]RustedUI eliminates this completely:[/b] by computing all layout, styling, and text-wrapping inside a native [b]Rust engine (powered by Taffy v0.7 Flexbox & Grid)[/b] with arena memory and 0 bytes of C# garbage collection per frame, it outputs raw MonoGame [font=monospace]SpriteBatch[/font] draw commands in microseconds ([b]~0.03–0.08 ms[/b]).

Whether you are a [b]player[/b] who wants beautiful, customizable HUDs and menus that feel like native Stardew Valley, or a [b]mod author[/b] looking to build cutting-edge menus using HTML, Tailwind CSS, React/TSX, or C# Fluent Builders, RustedUI is built for you!

[line]

[size=5][b][color=#E69138]KEY FEATURES AT A GLANCE[/color][/b][/size]
[list]
[*] [b][color=#10B981]Zero Garbage Collection Overhead:[/color][/b] Pure 60/120+ FPS gameplay with 0 bytes of managed C# allocations per frame during rendering.
[*] [b][color=#F59E0B]10 Live Color Themes:[/color][/b]
[list]
[*] [b]Pelican Classic:[/b] Authentic vanilla Stardew wooden and parchment 9-slice boxes.
[*] [b]Shadcn Zinc Dark:[/b] Ultra-clean modern dark mode (#09090B / #18181B).
[*] [b]Wizard Arcane:[/b] Mystic deep violet and starlight gold.
[*] [b]JojaCorp Blue:[/b] Industrial corporate cobalt and cyan.
[*] [b]Junimo Forest:[/b] Organic emerald and dewdrop green.
[*] [b]Catppuccin Mocha, Tokyo Night, Dracula, Nord & Cyberpunk Neon:[/b] Vibrant aesthetic palettes!
[/list]
[*] [b][color=#60A5FA]Theme-Aware Floating Tooltips:[/color][/b] Hovering over any HUD element or item slot shows an interactive tooltip styled to match your selected theme.
[*] [b][color=#A78BFA]Per-Mod Theme Isolation:[/color][/b] Third-party mods keep their own custom visual branding without being overridden by global theme changes (unless explicitly opted in).
[*] [b][color=#EC4899]Steam Deck & Gamepad Ready:[/color][/b] Native controller support, full-screen UI scaling, and proton compatibility with no third-party overlay hooks.
[*] [b][color=#38BDF8]Rich Built-In Widget Catalog:[/color][/b] Inventory item slots with quality stars, friendship heart meters, coin counters, progress bars, text input with keyboard support, smooth sliders, and hardware-clipped scroll views.
[*] [b][color=#F43F5E]Ergonomic Modder Tooling:[/color][/b] Build menus with [b]HTML + Tailwind CSS[/b] (with live in-game hot-reload), [b]React/Vite TSX[/b] ahead-of-time components, or a type-safe [b]GPUI-style C# Fluent Builder[/b].
[/list]

[line]

[size=5][b][color=#E69138]DEFAULT KEYBINDS[/color][/b][/size]
[table]
[tr]
[th][b]Keybind[/b][/th]
[th][b]Action[/b][/th]
[th][b]Description[/b][/th]
[/tr]
[tr]
[td][b][color=#10B981]F8[/color][/b][/td]
[td][b]Open Control Panel[/b][/td]
[td]Opens the RustedUI Control Panel, Theme Switcher, Audio Toggles, and Feature Test Lab. Can also be opened on the Title Screen![/td]
[/tr]
[tr]
[td][b][color=#60A5FA]F7[/color][/b][/td]
[td][b]Toggle Mini-HUD[/b][/td]
[td]Quickly shows or hides the in-game telemetry & quick-action overlay (Disabled by default).[/td]
[/tr]
[tr]
[td][b][color=#F59E0B]ESC / Right-Click[/color][/b][/td]
[td][b]Close Menu[/b][/td]
[td]Closes active RustedUI menus and returns control to the game.[/td]
[/tr]
[/table]

[line]

[size=5][b][color=#E69138]INSTALLATION[/color][/b][/size]
[list=1]
[*] Install the latest version of [url=https://www.nexusmods.com/stardewvalley/mods/2400][b]SMAPI (4.0.0 or higher)[/b][/url].
[*] Download the [b]RustedUI[/b] zip file using [b]Vortex[/b] or [b]Manual Download[/b].
[*] If installing manually, extract the archive into your [font=monospace]Stardew Valley/Mods/[/font] directory.
[*] Launch the game using SMAPI. Press [b]F8[/b] in-game or on the title screen to verify installation!
[*] [i](Optional)[/i] If you have [b]Generic Mod Config Menu (GMCM)[/b] installed, you can configure hotkeys and HUD visibility from the in-game settings menu.
[/list]

[line]

[size=5][b][color=#E69138]SECURITY & NATIVE BINARIES DISCLOSURE[/color][/b][/size]
[quote]
RustedUI includes pre-compiled native libraries ([font=monospace]librusted_bridge.so[/font] for Linux and [font=monospace]rusted_bridge.dll[/font] for Windows / Steam Deck Proton).
[list]
[*] [b]Pure CPU Layout Math:[/b] The native library performs strictly in-memory layout computations (Taffy Flexbox/Grid).
[*] [b]Zero Network / Zero OS Access:[/b] It makes 0 network calls, touches no disk/file systems, and collects 0 telemetry.
[*] [b]No VC++ Redistributable Needed:[/b] Compiled with static CRT ([font=monospace]+crt-static[/font]) for 100% out-of-the-box Proton / Wine compatibility.
[*] [b]Public Source Code:[/b] Fully open for audit on [url=https://github.com/MehmetCanWT/Rusted][b]GitHub[/b][/url].
[/list]
[/quote]

[line]

[size=5][b][color=#E69138]LICENSE & PERMISSIONS[/color][/b][/size]
RustedUI is distributed under the [b]RustedUI Source-Available Software License (v1.0)[/b]:
[list]
[*] [b]Free for Players:[/b] 100% free to download, install, and enjoy.
[*] [b]Free for Mod Authors:[/b] You are free to use RustedUI as a library or dependency in your own Stardew Valley mods without paying any royalties or fees.
[*] [b]Redistribution Restricted:[/b] Re-hosting, mirror uploading, or commercial redistribution on third-party sites or paywalled services is strictly prohibited without an explicit commercial license agreement.
[/list]

[center]
[line]
[i][color=#A0AEC0]Created with ❤️ for the Stardew Valley Modding Community.[/color][/i]
[/center]
```
