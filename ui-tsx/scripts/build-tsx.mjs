#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(__dirname, "../../src/Rusted.Smapi/assets/ui");

function escapeXml(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function h(tag, props, ...children) {
  if (typeof tag === "function") {
    return tag({ ...(props ?? {}), children });
  }
  const flat = children.flat(Infinity).filter((c) => c !== null && c !== undefined && c !== false);
  return { tag, props: props ?? {}, children: flat };
}

function render(node, indent = 0) {
  const pad = " ".repeat(indent);
  const attrs = Object.entries(node.props)
    .filter(([k, v]) => k !== "children" && v !== undefined && v !== null && v !== false)
    .map(([k, v]) => {
      const name = k === "className" ? "class" : k === "onClick" ? "on:click" : k;
      return `${name}="${escapeXml(v)}"`;
    })
    .join(" ");

  const open = attrs.length > 0 ? `<${node.tag} ${attrs}` : `<${node.tag}`;
  if (node.children.length === 0) return `${pad}${open} />`;
  if (node.children.length === 1 && typeof node.children[0] === "string") {
    return `${pad}${open}>${escapeXml(node.children[0])}</${node.tag}>`;
  }
  const body = node.children
    .map((c) =>
      typeof c === "string" ? `${" ".repeat(indent + 4)}${escapeXml(c)}` : render(c, indent + 4)
    )
    .join("\n");
  return `${pad}${open}>\n${body}\n${pad}</${node.tag}>`;
}

// Reusable Shadcn & Lucide Primitives
const Icon = ({ name, size = "md" }) =>
  h("icon", { name, class: `lucide-icon lucide-${size}` });

const Badge = ({ id, variant = "primary", children = [] }, ...restChildren) =>
  h(
    "badge",
    { id, variant, class: `sdv-badge badge-${variant}` },
    ...(Array.isArray(children) ? children : [children]),
    ...restChildren
  );

const Switch = ({ id, checked = true, onClick, tooltip }) =>
  h("switch", { id, checked: checked ? "true" : "false", "on:click": onClick, tooltip });

const ThemeBar = () =>
  h(
    "div",
    { class: "flex flex-col w-full px-3 py-2 sdv-panel gap-2" },
    h(
      "div",
      { class: "flex flex-row items-center justify-between w-full gap-2" },
      h(
        "div",
        { class: "flex flex-row items-center gap-2 shrink-0" },
        Icon({ name: "palette", size: "md" }),
        h("span", { id: "lbl_active_theme", class: "font-small text-sdv-dark" }, "Theme: Pelican Classic")
      ),
      h(
        "span",
        { id: "lbl_mod_ram_cpu", class: "font-small text-sdv-brown shrink-0" },
        "Mod CPU: 0.1% | Rust RAM: 42 KB"
      )
    ),
    h(
      "div",
      { class: "flex flex-row items-center justify-between w-full gap-2" },
      h(
        "button",
        {
          id: "btn_theme_0",
          class: "sdv-btn flex-1",
          "on:click": "theme_pelican",
          tooltip: "Pelican Classic: Vanilla Stardew Wooden and Parchment 9-Slice Theme",
        },
        "Pelican"
      ),
      h(
        "button",
        {
          id: "btn_theme_1",
          class: "sdv-btn flex-1",
          "on:click": "theme_zinc",
          tooltip: "Shadcn Zinc Dark: Modern High-Contrast Dark Mode (#09090B / #18181B)",
        },
        "Zinc Dark"
      ),
      h(
        "button",
        {
          id: "btn_theme_2",
          class: "sdv-btn flex-1",
          "on:click": "theme_arcane",
          tooltip: "Wizard Arcane: Deep Violet and Starlight Gold Theme (#0F081D / #A855F7)",
        },
        "Arcane"
      ),
      h(
        "button",
        {
          id: "btn_theme_3",
          class: "sdv-btn flex-1",
          "on:click": "theme_joja",
          tooltip: "JojaCorp Blue: Industrial Cyber-Blue Theme (#0F172A / #38BDF8)",
        },
        "JojaCorp"
      ),
      h(
        "button",
        {
          id: "btn_theme_4",
          class: "sdv-btn flex-1",
          "on:click": "theme_junimo",
          tooltip: "Junimo Forest: Organic Emerald and Dewdrop Theme (#14261C / #86EFAC)",
        },
        "Junimo"
      )
    ),
    h(
      "div",
      { class: "flex flex-row items-center justify-between w-full gap-2" },
      h(
        "button",
        {
          id: "btn_theme_5",
          class: "sdv-btn flex-1",
          "on:click": "theme_catppuccin",
          tooltip: "Catppuccin Mocha: Soothing Pastel Dark Theme (#1E1E2E / #CBA6F7)",
        },
        "Catppuccin"
      ),
      h(
        "button",
        {
          id: "btn_theme_6",
          class: "sdv-btn flex-1",
          "on:click": "theme_tokyonight",
          tooltip: "Tokyo Night: Clean Neon-Lit Downtown Tokyo Theme (#1A1B26 / #7AA2F7)",
        },
        "Tokyo Night"
      ),
      h(
        "button",
        {
          id: "btn_theme_7",
          class: "sdv-btn flex-1",
          "on:click": "theme_dracula",
          tooltip: "Dracula: Iconic Vampire Dark Palette (#282A36 / #FF79C6 / #BD93F9)",
        },
        "Dracula"
      ),
      h(
        "button",
        {
          id: "btn_theme_8",
          class: "sdv-btn flex-1",
          "on:click": "theme_nord",
          tooltip: "Nord Polar: Arctic Frost Bluish-Grey Palette (#2E3440 / #88C0D0)",
        },
        "Nord Polar"
      ),
      h(
        "button",
        {
          id: "btn_theme_9",
          class: "sdv-btn flex-1",
          "on:click": "theme_cyberpunk",
          tooltip: "Cyberpunk Neon: High-Voltage Night City Palette (#0A0616 / #FCEE0A / #00F0FF)",
        },
        "Cyberpunk"
      )
    )
  );

// 1. HUD Overlay (test_hud.html)
const HudTemplate = () =>
  h(
    "div",
    { class: "w-full h-full flex flex-col justify-start items-start p-4" },
    h(
      "div",
      {
        id: "hud_card",
        class: "flex flex-col w-360 h-auto sdv-box p-4 gap-2",
        tooltip: "RustedUI Zero-GC HUD Overlay (Rendered via Rust + MonoGame)",
      },
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full gap-2" },
        h(
          "div",
          { class: "flex flex-row items-center gap-2 shrink-0" },
          Icon({ name: "zap", size: "md" }),
          h("span", { class: "font-dialogue text-sdv-brown shadow" }, "RustedUI")
        ),
        Badge({ id: "hud_perf", variant: "secondary" }, "0.04ms")
      ),
      h("progress-bar", { id: "hud_bar", value: "80", max: "100", class: "w-full h-5" }),
      h(
        "button",
        {
          id: "btn_open_menu",
          class: "sdv-btn w-full",
          "on:click": "open_menu",
          tooltip: "Click to open the RustedUI Control Panel",
        },
        "Control Panel"
      )
    )
  );

// 2. Main Control Panel [F8] (test_menu.html)
const MainMenuTemplate = () =>
  h(
    "div",
    { class: "w-full h-full flex flex-col justify-center items-center" },
    h(
      "div",
      { id: "main_window", class: "flex flex-col w-960 h-auto sdv-box gap-3" },
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full gap-3" },
        h(
          "banner",
          { class: "sdv-parchment flex-row items-center gap-2" },
          Icon({ name: "cpu", size: "md" }),
          h("span", { class: "font-dialogue text-sdv-brown shadow" }, "RustedUI - Control Panel")
        ),
        h(
          "div",
          { class: "flex flex-row items-center gap-2 shrink-0" },
          Badge({ id: "bdg_layer", variant: "success" }, "TSX + Shadcn"),
          h("close-button", { id: "btn_close", "on:click": "close_menu", tooltip: "Close menu [ESC]" })
        )
      ),
      h(ThemeBar),
      h("hr", { class: "sdv-divider" }),
      h(
        "div",
        { class: "flex flex-row gap-4 w-full" },
        h(
          "panel",
          { class: "flex flex-col flex-1 gap-3 sdv-panel" },
          h(
            "div",
            { class: "flex flex-row items-center gap-2 w-full" },
            Icon({ name: "sliders", size: "md" }),
            h("span", { class: "font-dialogue text-sdv-brown" }, "1. Telemetry & HUD")
          ),
          h("span", { id: "lbl_render_time", class: "font-small text-sdv-dark" }, "Render Time: 0.08 ms (0 Bytes GC)"),
          h("span", { id: "lbl_cmd_count", class: "font-small text-sdv-dark" }, "Draw Commands: 0 | Events: 0"),
          h("span", { id: "lbl_ram_usage", class: "font-small text-sdv-brown" }, "CPU: 0.1% | Rust UI RAM: 42 KB"),
          h("hr", { class: "sdv-divider" }),
          h(
            "div",
            { class: "flex flex-row items-center justify-between w-full gap-2" },
            h("span", { class: "font-small text-sdv-dark" }, "In-Game Mini-HUD Overlay"),
            Switch({
              id: "chk_hud",
              checked: true,
              onClick: "toggle_hud",
              tooltip: "Toggle the top-left RustedUI HUD overlay",
            })
          ),
          h(
            "div",
            { class: "flex flex-row items-center justify-between w-full gap-2" },
            h("span", { class: "font-small text-sdv-dark" }, "Vanilla UI Sound Effects"),
            Switch({
              id: "chk_sound",
              checked: true,
              onClick: "toggle_sound",
              tooltip: "Toggle vanilla Stardew UI sound cues",
            })
          ),
          h(
            "div",
            { class: "flex flex-col gap-1 w-full" },
            h(
              "div",
              { class: "flex flex-row justify-between items-center w-full" },
              h("span", { class: "font-small text-sdv-dark" }, "HUD Progress Bar:"),
              h("span", { id: "lbl_energy_val", class: "font-small text-sdv-brown" }, "80 / 100")
            ),
            h("progress-bar", { id: "bar_energy", value: "80", max: "100", class: "w-full h-6" })
          )
        ),
        h(
          "panel",
          { class: "flex flex-col flex-1 gap-3 sdv-panel" },
          h(
            "div",
            { class: "flex flex-row items-center gap-2 w-full" },
            Icon({ name: "layers", size: "md" }),
            h("span", { class: "font-dialogue text-sdv-brown" }, "2. Menu Navigation")
          ),
          h(
            "span",
            { class: "font-small text-sdv-dark" },
            "Open the Credits & Architecture screen or launch the All-Features Test Lab with 5 live themes."
          ),
          h(
            "div",
            { class: "flex flex-row items-center gap-2 w-full" },
            h("coin-icon", {}),
            h(
              "span",
              { id: "lbl_slot_info", class: "font-small text-sdv-brown flex-1" },
              "Ready: Click below to open Credits or add +500G!"
            )
          ),
          h(
            "div",
            { class: "flex flex-col gap-2 w-full" },
            h(
              "button",
              {
                id: "btn_open_credits",
                class: "sdv-btn w-full",
                "on:click": "open_credits",
                tooltip: "Open RustedUI Credits & Architecture menu",
              },
              "Credits & About ->"
            ),
            h(
              "button",
              {
                id: "btn_open_test_direct",
                class: "sdv-btn w-full",
                "on:click": "open_feature_test",
                tooltip: "Open the All-Features Test Lab directly",
              },
              "Open All-Features Test Lab ->"
            ),
            h(
              "button",
              {
                id: "btn_give_gold",
                class: "sdv-btn w-full",
                "on:click": "give_gold",
                tooltip: "Add 500G to the farmer's wallet",
              },
              "+500G Add Gold"
            )
          )
        )
      ),
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full gap-2" },
        h(
          "span",
          { id: "lbl_hotreload_hint", class: "font-small text-sdv-muted flex-1" },
          "Status: Compiled from TSX + Tailwind + Shadcn + Lucide pipeline."
        ),
        h("span", { class: "font-small text-sdv-brown" }, "[ESC: Close]")
      )
    )
  );

// 3. Credits Menu (credits_menu.html)
const CreditsMenuTemplate = () =>
  h(
    "div",
    { class: "w-full h-full flex flex-col justify-center items-center" },
    h(
      "div",
      { id: "credits_window", class: "flex flex-col w-960 h-auto sdv-box gap-3" },
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full gap-3" },
        h(
          "banner",
          { class: "sdv-parchment flex-row items-center gap-2" },
          Icon({ name: "sparkles", size: "md" }),
          h("span", { class: "font-dialogue text-sdv-brown shadow" }, "RustedUI - Credits & Architecture")
        ),
        h("close-button", { id: "btn_close", "on:click": "close_menu", tooltip: "Close [ESC]" })
      ),
      h(ThemeBar),
      h("hr", { class: "sdv-divider" }),
      h(
        "panel",
        { class: "flex flex-col gap-3 sdv-panel w-full" },
        h(
          "div",
          { class: "flex flex-col gap-2 w-full" },
          h("span", { class: "font-dialogue text-sdv-brown" }, "Rusted (RustedUI) Core Stack"),
          h(
            "div",
            { class: "flex flex-row gap-2 shrink-0" },
            Badge({ variant: "primary" }, "Rust + Taffy"),
            Badge({ variant: "success" }, "Shadcn + Lucide"),
            Badge({ variant: "secondary" }, "React TSX")
          )
        ),
        h(
          "span",
          { class: "font-small text-sdv-dark" },
          "- Layout & Arena Engine: Rust + SlotMap + Taffy v0.7 Flexbox/Grid"
        ),
        h(
          "span",
          { class: "font-small text-sdv-dark" },
          "- Design System: 5 Live Themes (Pelican, Shadcn Zinc Dark, Arcane, Joja, Junimo) + Lucide Pixel Atlas"
        ),
        h(
          "span",
          { class: "font-small text-sdv-dark" },
          "- Authoring Layers: React/Vite JSX/TSX Component Compiler + HTML/Tailwind Hot-Reload + GPUI C# Fluent API"
        ),
        h(
          "span",
          { class: "font-small text-sdv-dark" },
          "- Zero-GC FFI ABI: Blittable RenderSliceView & EventSliceView (0 Bytes GC/frame)"
        ),
        h("hr", { class: "sdv-divider" }),
        h(
          "div",
          { class: "flex flex-row items-center gap-3 w-full" },
          h("heart-rating", { id: "credits_hearts", hearts: "10", "max-hearts": "10" }),
          h(
            "span",
            { class: "font-small text-sdv-brown flex-1" },
            "Crafted for the Stardew Valley 1.6+ modding community."
          )
        )
      ),
      h(
        "div",
        { class: "flex flex-row justify-between items-center gap-4 w-full" },
        h(
          "button",
          {
            id: "btn_back_main",
            class: "sdv-btn",
            "on:click": "open_main_menu",
            tooltip: "Return to the Control Panel",
          },
          "<- Back to Control Panel"
        ),
        h(
          "button",
          {
            id: "btn_open_test_lab",
            class: "sdv-btn",
            "on:click": "open_feature_test",
            tooltip: "Open the interactive lab testing all RustedUI widgets",
          },
          "Open All-Features Test Lab ->"
        )
      )
    )
  );

// 4. All-Features Test Lab (feature_test_menu.html)
const FeatureTestMenuTemplate = () =>
  h(
    "div",
    { class: "w-full h-full flex flex-col justify-center items-center" },
    h(
      "div",
      { id: "test_lab_window", class: "flex flex-col w-960 h-auto sdv-box gap-3" },
      // Top Header + Tabs + Mode Switch
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full gap-2" },
        h(
          "banner",
          { class: "sdv-parchment flex-row items-center gap-2" },
          Icon({ name: "code", size: "md" }),
          h("span", { class: "font-dialogue text-sdv-brown shadow" }, "Test Lab")
        ),
        h(
          "div",
          { class: "flex flex-row items-center gap-2 shrink-0" },
          h(
            "tab",
            { id: "tab_all", active: "true", "on:click": "select_tab_all", tooltip: "Split View: Show both Inputs and Inventory panels" },
            "Split"
          ),
          h(
            "tab",
            { id: "tab_1", "on:click": "select_tab_1", tooltip: "Tab 1: Focus on Interactive Inputs & Controls" },
            "Inputs"
          ),
          h(
            "tab",
            { id: "tab_2", "on:click": "select_tab_2", tooltip: "Tab 2: Focus on Inventory Slots, Icons & ScrollView" },
            "Inventory"
          ),
          h(
            "button",
            {
              id: "btn_toggle_mode",
              class: "sdv-btn",
              "on:click": "toggle_mode",
              tooltip: "Switch between TSX/HTML Template and GPUI C# Fluent Builder",
            },
            "C# Mode"
          ),
          h("close-button", { id: "btn_close", "on:click": "close_menu", tooltip: "Close [ESC]" })
        )
      ),
      // Live 10-Theme Switcher Bar
      h(ThemeBar),
      h("hr", { class: "sdv-divider" }),
      // Two-Column Test Bench
      h(
        "div",
        { class: "flex flex-row gap-4 w-full" },
        // Left Column: Slider, Progress, TextInput, Hearts, Shadcn Switches
        h(
          "panel",
          { id: "panel_tab_1", class: "flex flex-col flex-1 gap-3 sdv-panel" },
          h(
            "div",
            { class: "flex flex-row items-center gap-2 w-full" },
            Icon({ name: "sliders", size: "md" }),
            h("span", { class: "font-dialogue text-sdv-brown" }, "1. Interactive Controls")
          ),
          h(
            "div",
            { class: "flex flex-col gap-1 w-full" },
            h(
              "div",
              { class: "flex flex-row justify-between items-center w-full" },
              h("span", { class: "font-small text-sdv-dark" }, "Draggable Slider & Bar:"),
              h("span", { id: "lbl_slider_val", class: "font-small text-sdv-brown" }, "80 / 100")
            ),
            h("slider", {
              id: "sld_energy",
              value: "80",
              max: "100",
              class: "w-full",
              "on:click": "slider_changed",
              tooltip: "Drag with left mouse button to adjust value!",
            }),
            h("progress-bar", {
              id: "bar_energy",
              value: "80",
              max: "100",
              class: "w-full h-5",
              tooltip: "Synchronized Progress Bar",
            })
          ),
          h(
            "div",
            { class: "flex flex-col gap-1 w-full" },
            h("span", { class: "font-small text-sdv-dark" }, "Keyboard Text Input & Trash:"),
            h(
              "div",
              { class: "flex flex-row items-center gap-2 w-full" },
              h(
                "text-input",
                {
                  id: "inp_search",
                  class: "flex-1",
                  "on:click": "focus_search",
                  tooltip: "Click and type on your keyboard!",
                },
                "Hello RustedUI!"
              ),
              h("trash-button", {
                id: "btn_clear_input",
                "on:click": "clear_input",
                tooltip: "Clear text input (Trashcan sound)",
              })
            )
          ),
          h(
            "div",
            { class: "flex flex-col gap-1 w-full" },
            h(
              "div",
              { class: "flex flex-row justify-between items-center w-full" },
              h("span", { class: "font-small text-sdv-dark" }, "Friendship Hearts:"),
              h("span", { id: "lbl_hearts_val", class: "font-small text-sdv-brown" }, "8 / 10 Hearts")
            ),
            h(
              "div",
              { class: "flex flex-row items-center justify-between w-full gap-2" },
              h("arrow-left", { id: "btn_heart_minus", "on:click": "heart_minus", tooltip: "Decrease hearts (-1)" }),
              h("heart-rating", {
                id: "heart_meter",
                hearts: "8",
                "max-hearts": "10",
                tooltip: "Dynamic Friendship Heart Meter",
              }),
              h("arrow-right", { id: "btn_heart_plus", "on:click": "heart_plus", tooltip: "Increase hearts (+1)" })
            )
          ),
          h(
            "div",
            { class: "flex flex-row items-center justify-between w-full gap-2" },
            h(
              "div",
              { class: "flex flex-row items-center gap-2" },
              Switch({ id: "chk_hud", checked: true, onClick: "toggle_hud", tooltip: "Shadcn Switch: Toggle HUD" }),
              h("span", { class: "font-small text-sdv-dark" }, "HUD")
            ),
            h(
              "div",
              { class: "flex flex-row items-center gap-2" },
              h("checkbox", {
                id: "chk_sound",
                checked: "true",
                "on:click": "toggle_sound",
                tooltip: "Vanilla Checkbox: Toggle UI Sounds",
              }),
              h("span", { class: "font-small text-sdv-dark" }, "SFX")
            ),
            h("coin-icon", { tooltip: "Vanilla Coin Sprite" }),
            h("button", { id: "btn_gold", class: "sdv-btn", "on:click": "give_gold", tooltip: "Add +500G" }, "+500G")
          )
        ),
        // Right Column: Inventory Slots, Badges, Lucide Icons & Hardware Scissor ScrollView
        h(
          "panel",
          { id: "panel_tab_2", class: "flex flex-col flex-1 gap-3 sdv-panel" },
          h(
            "div",
            { class: "flex flex-row items-center gap-2 w-full" },
            Icon({ name: "package", size: "md" }),
            h("span", { class: "font-dialogue text-sdv-brown" }, "2. Slots, Icons & Scroll")
          ),
          h(
            "div",
            { class: "flex flex-row gap-2 w-full justify-between" },
            h("item-slot", {
              id: "slot_prismatic",
              item: "74",
              count: "7",
              quality: "iridium",
              "on:click": "slot_prismatic",
              tooltip: "Prismatic Shard (Iridium Quality) - Click to inspect!",
            }),
            h("item-slot", {
              id: "slot_starfruit",
              item: "268",
              count: "24",
              quality: "gold",
              "on:click": "slot_starfruit",
              tooltip: "Starfruit (Gold Quality) - Click to inspect!",
            }),
            h("item-slot", {
              id: "slot_iridium",
              item: "337",
              count: "50",
              quality: "iridium",
              "on:click": "slot_iridium",
              tooltip: "Iridium Bar (50 Stack) - Click to inspect!",
            }),
            h("item-slot", {
              id: "slot_ancient",
              item: "454",
              count: "12",
              quality: "silver",
              "on:click": "slot_ancient",
              tooltip: "Ancient Fruit (Silver Quality) - Click to inspect!",
            }),
            h("item-slot", {
              id: "slot_diamond",
              item: "72",
              count: "99",
              quality: "iridium",
              "on:click": "slot_diamond",
              tooltip: "Diamond (99 Stack) - Click to inspect!",
            })
          ),
          h(
            "div",
            { class: "flex flex-row items-center justify-between w-full gap-2" },
            h(
              "div",
              { class: "flex flex-row items-center gap-2" },
              Icon({ name: "sparkles", size: "sm" }),
              Icon({ name: "shield", size: "sm" }),
              Icon({ name: "sword", size: "sm" }),
              Icon({ name: "flame", size: "sm" }),
              Icon({ name: "terminal", size: "sm" }),
              Icon({ name: "settings", size: "sm" })
            ),
            h(
              "div",
              { class: "flex flex-row items-center gap-1" },
              Badge({ variant: "secondary" }, "Shadcn"),
              Badge({ variant: "destructive" }, "Alert")
            )
          ),
          h(
            "span",
            { id: "lbl_slot_info", class: "font-small text-sdv-brown" },
            "Hover over any item slot or icon to test floating tooltips!"
          ),
          h(
            "div",
            { class: "flex flex-row justify-between items-center w-full" },
            h("span", { class: "font-small text-sdv-dark" }, "Clipped ScrollView (Wheel & Arrows):"),
            h(
              "div",
              { class: "flex flex-row gap-2" },
              h("arrow-up", { id: "btn_scroll_up", "on:click": "scroll_up", tooltip: "Scroll Up" }),
              h("arrow-down", { id: "btn_scroll_down", "on:click": "scroll_down", tooltip: "Scroll Down" })
            )
          ),
          h(
            "scroll-view",
            {
              id: "log_scroll",
              class: "w-full h-150 sdv-panel",
              tooltip: "Scroll with the mouse wheel while hovering here!",
            },
            h("span", { class: "font-small text-sdv-dark" }, "[01] RustedUI Taffy v0.7 Flexbox & Grid engine active."),
            h("span", { class: "font-small text-sdv-dark" }, "[02] Hardware ScissorRectangle restores CullNone on PopClip."),
            h("span", { class: "font-small text-sdv-dark" }, "[03] 10 Themes: Pelican, Zinc, Arcane, Joja, Junimo, Mocha, Tokyo, Dracula, Nord, Cyber."),
            h("span", { class: "font-small text-sdv-dark" }, "[04] Per-Mod Theme Isolation: External mods keep own theme unless opted in."),
            h("span", { class: "font-small text-sdv-dark" }, "[05] Theme-aware Floating Tooltips on HUD and all menus."),
            h("span", { class: "font-small text-sdv-dark" }, "[06] React/Vite JSX/TSX Component Authoring Pipeline."),
            h("span", { class: "font-small text-sdv-dark" }, "[07] OptionsSlider continuous drag & keyboard TextInput."),
            h("span", { id: "lbl_ram_usage", class: "font-small text-sdv-brown" }, "[08] CPU: 0.1% | Rust UI RAM: 42 KB"),
            h("span", { id: "lbl_render_time", class: "font-small text-sdv-brown" }, "[09] Render: 0.08ms | 0 Bytes GC")
          )
        )
      ),
      // Footer Navigation & Telemetry
      h(
        "div",
        { class: "flex flex-row justify-between items-center w-full" },
        h(
          "button",
          {
            id: "btn_back_credits",
            class: "sdv-btn",
            "on:click": "open_credits",
            tooltip: "Return to the Credits & Architecture menu",
          },
          "<- Back to Credits"
        ),
        h(
          "span",
          { id: "lbl_cmd_count", class: "font-small text-sdv-muted" },
          "Draw Commands: 0 | Events: 0 | Mode: TSX + Shadcn + Lucide"
        )
      )
    )
  );

fs.mkdirSync(outDir, { recursive: true });

const outputs = [
  ["test_hud.html", HudTemplate()],
  ["test_menu.html", MainMenuTemplate()],
  ["credits_menu.html", CreditsMenuTemplate()],
  ["feature_test_menu.html", FeatureTestMenuTemplate()],
];

for (const [filename, vnode] of outputs) {
  const html = `<!-- Auto-generated from ui-tsx JSX/TSX Component Pipeline -->\n${render(vnode)}\n`;
  fs.writeFileSync(path.join(outDir, filename), html, "utf8");
  console.log(`  -> Compiled TSX component to assets/ui/${filename}`);
}
