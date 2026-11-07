[center]
[size=7][b][color=#F5D597]RUSTED[/color] [color=#E69138]UI[/color][/b][/size]
[size=4][i][color=#DCDCDC]Next-Generation High-Performance In-Game UI & HUD Framework for Stardew Valley 1.6+[/color][/i][/size]

[size=3][b][color=#10B981]✦ ZERO GC ALLOCATION[/color] | [color=#60A5FA]✦ RUST TAFFY ENGINE[/color] | [color=#F59E0B]✦ 10 LIVE THEMES[/color] | [color=#A78BFA]✦ STEAM DECK READY[/color][/b][/size]

[line]
[url=https://github.com/MehmetCanWT/Rusted][img]https://img.shields.io/badge/GitHub-Repository-black?style=for-the-badge&logo=github[/img][/url] [url=https://www.nexusmods.com/stardewvalley/mods/2400][img]https://img.shields.io/badge/Requires-SMAPI%204.0+-brightgreen?style=for-the-badge[/img][/url] [img]https://img.shields.io/badge/Stardew%20Valley-1.6+-orange?style=for-the-badge[/img] [img]https://img.shields.io/badge/License-Source--Available-blue?style=for-the-badge[/img]
[line]
[/center]

[size=5][b][color=#E69138]Description[/color][/b][/size]
[b]RustedUI[/b] is an ultra-fast, zero-lag in-game user interface framework, telemetry HUD overlay, and visual showcase built specifically for [b]Stardew Valley 1.6+ (SMAPI 4.0+)[/b].

In heavily modded Stardew Valley setups, traditional C# UI mods frequently cause garbage collection (GC) micro-stutters and sudden frame spikes when rendering detailed menus or HUD overlays. [b]RustedUI completely eliminates this issue:[/b]
[list]
[*] It delegates all layout calculations, styling, and text-wrapping to an embedded native [b]Rust engine powered by Taffy v0.7[/b] (CSS Flexbox & Grid).
[*] Operating with arena-allocated memory and [b]0 bytes of managed C# garbage collection per frame[/b], it outputs flat command buffers directly to MonoGame's hardware [font=monospace]SpriteBatch[/font] in microseconds ([b]~0.03–0.08 ms[/b]).
[/list]

Whether you are a [b]player[/b] looking for a polished, customizable HUD overlay and snappy menus that blend with Stardew Valley's art style, or a [b]mod author[/b] seeking to build modern interfaces using HTML + Tailwind CSS, React/TSX, or a type-safe C# Fluent Builder, RustedUI is built for you!

[line]

[size=5][b][color=#E69138]Installation instructions[/color][/b][/size]
Follow these straightforward steps to install RustedUI:

[list=1]
[*] [b]Prerequisites:[/b] Make sure you have installed the latest version of [url=https://www.nexusmods.com/stardewvalley/mods/2400][b]SMAPI (4.0.0 or higher)[/b][/url] for Stardew Valley 1.6+.
[*] [b]Download:[/b] Download the [b]RustedUI[/b] archive via [b]Vortex[/b] or click [b]Manual Download[/b].
[*] [b]Extract:[/b] If installing manually, extract the downloaded zip file into your [font=monospace]Stardew Valley/Mods/[/font] directory so that you have a [font=monospace]Mods/RustedUI/[/font] folder containing [font=monospace]manifest.json[/font].
[*] [b]Launch & Verify:[/b] Launch the game using SMAPI. Press [b]F8[/b] anywhere (in-game or on the Title Screen) to open the RustedUI Control Panel!
[/list]

[b]Default Controls & Shortcuts:[/b]
[table]
[tr]
[th][b]Keybind[/b][/th]
[th][b]Action[/b][/th]
[th][b]Description[/b][/th]
[/tr]
[tr]
[td][b][color=#10B981]F8[/color][/b][/td]
[td][b]Open Control Panel[/b][/td]
[td]Opens the RustedUI Control Panel, Theme Switcher, Audio Toggles, and Feature Test Lab. (Can also be opened on the Title Screen!)[/td]
[/tr]
[tr]
[td][b][color=#60A5FA]F7[/color][/b][/td]
[td][b]Toggle Mini-HUD[/b][/td]
[td]Quickly toggles the in-game telemetry & quick-action HUD overlay on/off (Disabled by default).[/td]
[/tr]
[tr]
[td][b][color=#F59E0B]ESC / Right-Click[/color][/b][/td]
[td][b]Close Menu[/b][/td]
[td]Closes active RustedUI menus and returns input to the game.[/td]
[/tr]
[/table]

[line]

[size=5][b][color=#E69138]Main features[/color][/b][/size]
[list]
[*] [b][color=#10B981]Zero Garbage Collection Overhead:[/color][/b] Pure 60/120+ FPS gameplay with 0 bytes of managed C# allocations per frame while rendering.
[*] [b][color=#F59E0B]10 Live In-Game Themes:[/color][/b]
[list]
[*] [b]Pelican Classic:[/b] Authentic vanilla Stardew wooden and parchment 9-slice boxes.
[*] [b]Shadcn Zinc Dark:[/b] Ultra-clean modern dark mode (#09090B / #18181B).
[*] [b]Wizard Arcane:[/b] Mystic deep violet and starlight gold.
[*] [b]JojaCorp Blue:[/b] Industrial corporate cobalt and electric cyan.
[*] [b]Junimo Forest:[/b] Organic emerald and dewdrop green.
[*] [b]Catppuccin Mocha, Tokyo Night, Dracula, Nord & Cyberpunk Neon:[/b] Vibrant aesthetic palettes!
[/list]
[*] [b][color=#60A5FA]Theme-Aware Floating Tooltips:[/color][/b] Hovering over any HUD element or item slot shows an interactive tooltip styled to match the active theme tokens.
[*] [b][color=#A78BFA]Per-Mod Theme Isolation:[/color][/b] Third-party mods retain their unique aesthetic without being overridden by global theme changes.
[*] [b][color=#EC4899]Steam Deck & Gamepad Ready:[/color][/b] Native controller support, full-screen UI scaling, and out-of-the-box Proton / Linux compatibility.
[*] [b][color=#38BDF8]Rich Built-In Widget Catalog:[/color][/b] Inventory item slots with quality stars, friendship heart meters, coin counters, progress bars, text input with keyboard support, smooth sliders, and hardware-clipped scroll views.
[*] [b][color=#F43F5E]Ergonomic Modder Tooling:[/color][/b] Build menus with [b]HTML + Tailwind CSS[/b] (with live in-game hot-reload), [b]React/Vite TSX[/b] ahead-of-time components, or a type-safe [b]GPUI-style C# Fluent Builder[/b].
[/list]

[line]

[size=5][b][color=#E69138]Requirements[/color][/b][/size]
RustedUI is lightweight and fully self-contained:

[list]
[*] [b]Strictly Required:[/b]
[list]
[*] [b]Stardew Valley 1.6+[/b] (PC / Steam / GOG).
[*] [url=https://www.nexusmods.com/stardewvalley/mods/2400][b]SMAPI - Stardew Modding API (Version 4.0.0 or higher)[/b][/url] (Mod ID: 2400).
[/list]
[*] [b]Optional Integration:[/b]
[list]
[*] [url=https://www.nexusmods.com/stardewvalley/mods/5098][b]Generic Mod Config Menu (GMCM)[/b][/url] (Mod ID: 5098): Allows configuring hotkeys (F7/F8) and UI sound toggles directly in Stardew Valley's in-game options menu.
[/list]
[*] [b]No External Runtimes Needed:[/b]
RustedUI includes pre-compiled, self-contained native libraries ([font=monospace]librusted_bridge.so[/font] for Linux/Deck and [font=monospace]rusted_bridge.dll[/font] statically linked with [font=monospace]+crt-static[/font] for Windows/Proton). You do not need to install any Visual C++ redistributable or Rust compiler.
[/list]

[line]

[size=5][b][color=#E69138]Shout outs[/color][/b][/size]
Special thanks and heartfelt appreciation to the incredible creators and open-source projects that made RustedUI possible:

[list]
[*] [b]ConcernedApe (Eric Barone):[/b] For creating Stardew Valley and giving the modding community endless inspiration and support with the 1.6 update.
[*] [b]Pathoschild:[/b] For SMAPI, [font=monospace]SMAPI.ModBuildConfig[/font], and countless years of dedicated leadership in Stardew Valley modding infrastructure.
[*] [b]The Dioxus Labs Team:[/b] For [url=https://github.com/DioxusLabs/taffy][b]Taffy[/b][/url], the blazing-fast Rust implementation of CSS Flexbox and CSS Grid.
[*] [b]spacechase0:[/b] For Generic Mod Config Menu (GMCM), the gold standard for in-game mod configuration.
[*] [b]The Rust & MonoGame Communities:[/b] For providing unmatched performance tooling and robust game engine foundations.
[/list]

[line]

[size=5][b][color=#E69138]Security & Native Binaries Disclosure[/color][/b][/size]
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

[size=5][b][color=#E69138]License & Permissions[/color][/b][/size]
RustedUI is distributed under the [b]RustedUI Source-Available Software License (v1.0)[/b]:
[list]
[*] [b]Free for Players:[/b] 100% free to download, install, and enjoy.
[*] [b]Free for Mod Authors:[/b] You are free to use RustedUI as a library or dependency in your own Stardew Valley mods without paying any royalties or fees.
[*] [b]Redistribution Restricted:[/b] Re-hosting, mirror uploading, or commercial redistribution on third-party sites or paywalled services is strictly prohibited without an explicit commercial license agreement.
[/list]

[center]
[line]
[i][color=#A0AEC0]Crafted with ❤️ for the Stardew Valley Modding Community.[/color][/i]
[/center]
