# RustedUI — HTML / XML & Tailwind CSS Authoring Guide

RustedUI includes a zero-GC HTML/XML template parser (`quick-xml`) and a compile/load-time **Tailwind CSS utility parser** (`crates/rusted-core/src/tailwind.rs`) that maps utility classes directly onto `taffy::Style` structs and `ComponentFlags`.

---

## 1. Loading HTML Templates with Live Hot-Reload

In your SMAPI mod, load any `.html` file using `RustedView`:

```csharp
using System.IO;
using Rusted.Smapi.UI;

var view = new RustedView(ownerModId: ModManifest.UniqueID);
string htmlPath = Path.Combine(Helper.DirectoryPath, "assets", "ui", "my_menu.html");

// Automatically watches the file and hot-reloads in <20ms whenever saved!
view.LoadMarkupFileWithHotReload(htmlPath);
```

When you edit and save `my_menu.html` while Stardew Valley is running, `FileSystemWatcher` reloads the layout into the Rust `UiTree` while **preserving live widget state** (slider values, checkbox states, item slots, hearts, and scroll offsets) for matching element `id`s!

---

## 2. Supported Semantic HTML Tags & Components

| Tag | Description | Key Attributes |
|---|---|---|
| `<div>` | Flexbox / Block container (`sdv-box` adds outer window frame) | `id`, `class`, `tooltip`, `theme`, `use-rusted-theme` |
| `<panel>` | Inset wooden panel (Pelican) or card surface (Modern themes) | `id`, `class`, `tooltip` |
| `<card>` | Shadcn/UI Card container (`shadcn-card`) | `id`, `class`, `tooltip` |
| `<banner>` | Parchment scroll title banner (`sdv-parchment`) | `id`, `class` |
| `<span>`, `<p>`, `<label>` | Wrapping text node with automatic pixel-font measurement | `id`, `class` (`font-small`, `font-dialogue`, `font-tiny`) |
| `<button>` | Interactive button with hover highlight & click sound (`bigSelect`) | `id`, `class`, `on:click`, `tooltip` |
| `<tab>` | Top/side navigation tab with active shift & state | `id`, `active="true\|false"`, `on:click`, `tooltip` |
| `<switch>` | Shadcn/UI pill toggle switch (`52x28` track + thumb) | `id`, `checked="true\|false"`, `on:click`, `tooltip` |
| `<checkbox>` | Vanilla Stardew `OptionsCheckbox` (`36x36`) | `id`, `checked="true\|false"`, `on:click`, `tooltip` |
| `<slider>` | Draggable continuous `OptionsSlider` (`0..max`) | `id`, `value`, `max`, `on:click`, `tooltip` |
| `<progress-bar>` | Dynamic progress / energy bar with smooth ratio fill | `id`, `value`, `max`, `class`, `tooltip` |
| `<text-input>` | Interactive keyboard text box with focus state | `id`, `class`, `on:click`, `tooltip` |
| `<item-slot>` | `64x64` inventory slot with object sprite, quality star & stack count | `id`, `item="74"`, `count="7"`, `quality="silver\|gold\|iridium"`, `on:click`, `tooltip` |
| `<heart-rating>` | Friendship heart meter (`1..14` hearts, filled vs empty) | `id`, `hearts="8"`, `max-hearts="10"`, `tooltip` |
| `<scroll-view>` | Hardware `ScissorRectangle`-clipped scrollable container + scrollbar | `id`, `class` (`h-150`), `tooltip` |
| `<badge>` | Shadcn/UI status badge | `id`, `variant="primary\|secondary\|outline\|destructive\|success"` |
| `<icon>` | Crisp `16x16` Lucide pixel-art icon from `TextureId::LucideAtlas` | `name="sparkles\|cpu\|sliders\|layers\|palette\|zap\|package\|shield\|sword\|flame\|terminal\|settings\|code\|heart"`, `class="lucide-sm\|lucide-md\|lucide-lg"` |
| `<close-button>` | Upper-right red `X` close button (`drumkit6` sound) | `id`, `on:click="close_menu"`, `tooltip` |
| `<trash-button>` | Animated trash can clear/delete button (`trashcan` sound) | `id`, `on:click`, `tooltip` |
| `<arrow-left>`, `<arrow-right>`, `<arrow-up>`, `<arrow-down>` | Directional navigation / stepper arrow buttons | `id`, `on:click`, `tooltip` |
| `<coin-icon>` | Vanilla gold coin sprite (`24x24`) | `tooltip` |
| `<hr>` / `<divider>` | Horizontal wooden/themed separator line | `class="sdv-divider"` |

---

## 3. Supported Tailwind CSS Utility Classes

### Flexbox & Layout
- `flex`, `hidden`, `flex-row`, `flex-col`, `flex-wrap`
- `flex-1`, `grow`, `shrink-0`
- `items-start`, `items-center`, `items-end`, `items-stretch`
- `justify-start`, `justify-center`, `justify-end`, `justify-between`, `justify-around`, `justify-evenly`

### Sizing (`px` and `%`)
- `w-full` (`100%`), `h-full` (`100%`), `w-auto`, `h-auto`, `min-w-0`, `min-h-0`
- Tailwind spacing scale: `w-4`..`w-96`, `h-4`..`h-96`
- Direct pixel widths/heights: `w-360`, `w-440`, `w-640`, `w-800`, `w-900`, `w-960`, `h-150`, `h-160`, `h-480`, `h-600`, `h-640`

### Spacing (Padding, Margin, Gap)
- `p-0`..`p-8`, `px-1`..`px-8`, `py-1`..`py-8`
- `m-0`..`m-8`, `mt-1`..`mt-6`, `mb-1`..`mb-6`
- `gap-0`..`gap-8` (`gap-1` = `4px`, `gap-2` = `8px`, `gap-3` = `12px`, `gap-4` = `16px`)

### Typography & Colors
- **Fonts**: `font-small` (`Game1.smallFont`), `font-dialogue` (`Game1.dialogueFont`), `font-tiny` (`Game1.tinyFont`)
- **Text Effects**: `shadow` / `text-shadow`
- **Text Colors**: `text-sdv-dark`, `text-sdv-brown`, `text-sdv-gold`, `text-sdv-iridium`, `text-sdv-green`, `text-sdv-red`, `text-sdv-muted`, `text-white` (automatically mapped to high-contrast theme tokens when a dark/modern theme is active!)

---

## 4. Complete HTML Template Example

```xml
<div id="custom_mod_window" class="w-full h-full flex flex-col justify-center items-center">
    <div class="flex flex-col w-640 h-auto sdv-box gap-3">
        <div class="flex flex-row justify-between items-center w-full gap-2">
            <banner class="sdv-parchment flex-row items-center gap-2">
                <icon name="sparkles" class="lucide-md" />
                <span class="font-dialogue text-sdv-brown shadow">Alchemist Workbench</span>
            </banner>
            <close-button id="btn_close" on:click="close_menu" tooltip="Close Workbench [ESC]" />
        </div>

        <hr class="sdv-divider" />

        <panel class="flex flex-col gap-3 sdv-panel w-full">
            <div class="flex flex-row items-center justify-between w-full">
                <item-slot id="slot_input" item="74" count="3" quality="iridium" on:click="inspect_shard" tooltip="Prismatic Catalyst" />
                <div class="flex flex-col flex-1 gap-1 px-3">
                    <span id="lbl_status" class="font-small text-sdv-dark">Transmutation Charge: 80%</span>
                    <progress-bar id="bar_charge" value="80" max="100" class="w-full h-6" />
                </div>
                <button id="btn_transmute" class="sdv-btn" on:click="transmute" tooltip="Begin transmutation">Transmute</button>
            </div>
        </panel>
    </div>
</div>
```
