# CurseForge Description Template for RustedUI

> **How to Use in CurseForge:**
> 1. Open your project on CurseForge and go to **Description**.
> 2. Click the **`</>` (Source / HTML)** button in the top toolbar of the WYSIWYG editor.
> 3. Paste the HTML block below, then click `</>` again to switch back to the visual editor.
> 4. *(Note: This description contains NO mentions of Nexus Mods).*

```html
<div style="text-align: center;">
  <h1>⚡ RUSTED UI</h1>
  <p><em><strong>Next-Generation High-Performance In-Game UI &amp; HUD Engine for Stardew Valley 1.6+ (SMAPI 4.0+)</strong></em></p>
  <p><strong>✦ ZERO GC ALLOCATION &nbsp;|&nbsp; ✦ RUST TAFFY ENGINE &nbsp;|&nbsp; ✦ 10 LIVE THEMES &nbsp;|&nbsp; ✦ STEAM DECK VERIFIED</strong></p>
  <hr />
</div>

<h2>🌟 What is RustedUI?</h2>
<p>
  <strong>RustedUI</strong> is an ultra-fast, zero-lag in-game user interface framework and visual showcase for <strong>Stardew Valley 1.6+</strong>.
</p>
<p>
  Traditional UI mods in C# frequently suffer from garbage collection (GC) micro-stutters and frame spikes when drawing detailed menus or telemetry overlays. <strong>RustedUI eliminates this completely:</strong> by calculating all layout, styling, and text wrapping inside an embedded, compiled <strong>Rust engine (powered by Taffy v0.7 Flexbox &amp; Grid)</strong> with arena memory and 0 bytes of C# garbage collection per frame, it outputs raw MonoGame <code>SpriteBatch</code> draw calls in microseconds (<strong>~0.03–0.08 ms</strong>).
</p>
<p>
  Whether you are a <strong>player</strong> who wants clean, customizable HUDs and menus that match Stardew Valley's art style, or a <strong>mod author</strong> looking to build interfaces using HTML, Tailwind CSS, React/TSX, or C# Fluent Builders, RustedUI is built for you!
</p>

<hr />

<h2>✨ Key Features</h2>
<ul>
  <li><strong>🦀 Zero Garbage Collection Overhead:</strong> Smooth 60/120+ FPS gameplay with 0 bytes of managed C# allocations during rendering.</li>
  <li><strong>🎨 10 Instant In-Game Themes:</strong>
    <ul>
      <li><strong>Pelican Classic:</strong> Authentic vanilla Stardew Valley wooden and parchment 9-slice boxes.</li>
      <li><strong>Shadcn Zinc Dark:</strong> Sleek modern dark mode (<code>#09090B</code> / <code>#18181B</code>).</li>
      <li><strong>Wizard Arcane:</strong> Mystic deep violet and starlight gold.</li>
      <li><strong>JojaCorp Blue:</strong> Corporate industrial cobalt and cyan.</li>
      <li><strong>Junimo Forest:</strong> Organic emerald and dewdrop green.</li>
      <li><strong>Catppuccin Mocha, Tokyo Night, Dracula, Nord &amp; Cyberpunk Neon:</strong> Highly requested community palettes!</li>
    </ul>
  </li>
  <li><strong>💬 Theme-Aware Floating Tooltips:</strong> Hovering over any HUD element or item slot shows an interactive tooltip styled to match your selected theme.</li>
  <li><strong>🔒 Per-Mod Theme Isolation:</strong> Third-party mods keep their own custom branding without being overridden by global theme changes (unless explicitly opted in).</li>
  <li><strong>🎮 Steam Deck &amp; Gamepad Verified:</strong> Seamless controller support, full-screen UI scaling, and proton compatibility with no third-party overlay hooks.</li>
  <li><strong>📦 Built-In Widget Catalog:</strong> Item slots with quality stars, dynamic friendship heart meters, gold counters, progress bars, text input with keyboard support, smooth sliders, and hardware-clipped scroll views.</li>
  <li><strong>📐 Modder-Friendly Workflows:</strong> Build menus using <strong>HTML + Tailwind CSS</strong> (with live in-game hot-reload), <strong>React/Vite TSX</strong> ahead-of-time components, or a type-safe <strong>GPUI-style C# Fluent Builder</strong>.</li>
</ul>

<hr />

<h2>⌨️ Controls &amp; Shortcuts</h2>
<table border="1" cellpadding="8" style="border-collapse: collapse; width: 100%;">
  <thead>
    <tr style="background-color: rgba(255, 255, 255, 0.1);">
      <th>Keybind</th>
      <th>Function</th>
      <th>Details</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>F8</strong></td>
      <td><strong>RustedUI Control Panel</strong></td>
      <td>Opens the main control panel, theme selector, audio toggles, and feature test lab. Can also be opened on the Title Screen!</td>
    </tr>
    <tr>
      <td><strong>F7</strong></td>
      <td><strong>Toggle Mini-HUD</strong></td>
      <td>Quickly shows or hides the lightweight in-game telemetry &amp; quick-action overlay (Disabled by default).</td>
    </tr>
    <tr>
      <td><strong>ESC / Right-Click</strong></td>
      <td><strong>Close Menu</strong></td>
      <td>Closes active RustedUI menus and returns control to the game.</td>
    </tr>
  </tbody>
</table>

<hr />

<h2>📦 Installation</h2>
<ol>
  <li>Ensure you have <strong>SMAPI 4.0+</strong> installed for Stardew Valley 1.6+.</li>
  <li>Download the <strong>RustedUI</strong> zip file from this CurseForge page.</li>
  <li>Unzip the archive and place the <code>RustedUI</code> folder into your <code>Stardew Valley/Mods</code> folder.</li>
  <li>Launch the game through SMAPI. Press <strong>F8</strong> in-game or on the title screen to explore!</li>
  <li><em>(Optional)</em> If you have <strong>Generic Mod Config Menu (GMCM)</strong> installed, you can configure hotkeys and HUD visibility from the in-game settings menu.</li>
</ol>

<hr />

<h2>🛡️ Security &amp; Native Binary Transparency</h2>
<p>
  RustedUI includes pre-compiled, statically-linked native libraries (<code>librusted_bridge.so</code> for Linux and <code>rusted_bridge.dll</code> for Windows / Steam Deck Proton).
</p>
<blockquote>
  <p><strong>Security Disclosure:</strong></p>
  <ul>
    <li>The native library performs <strong>strictly in-memory math layout calculations</strong> (CSS Flexbox/Grid via the open-source Taffy engine).</li>
    <li>It makes <strong>zero network calls</strong>, touches no disk/file systems, executes no OS shell processes, and collects zero telemetry.</li>
    <li>It is compiled with static CRT (<code>+crt-static</code>), requiring no external Visual C++ runtime installs.</li>
    <li>Full source code is publicly audited and available on <a href="https://github.com/MehmetCanWT/Rusted" target="_blank">GitHub</a>.</li>
  </ul>
</blockquote>

<hr />

<h2>📄 License &amp; Permissions</h2>
<p>
  RustedUI is licensed under the <strong>RustedUI Source-Available Software License (v1.0)</strong>:
</p>
<ul>
  <li><strong>Free for Players:</strong> 100% free to download, install, and enjoy.</li>
  <li><strong>Free for Mod Authors:</strong> You are fully authorized and encouraged to use RustedUI as a library or dependency for your own Stardew Valley mods without any payment or royalties.</li>
  <li><strong>Redistribution Restricted:</strong> Direct re-hosting, mirror uploading, or commercial redistribution of RustedUI on third-party sites or paywalled platforms is strictly prohibited and requires an authorized commercial distribution agreement.</li>
</ul>

<hr />
<p style="text-align: center;"><em>Crafted with ❤️ for the Stardew Valley Modding Community.</em></p>
```
