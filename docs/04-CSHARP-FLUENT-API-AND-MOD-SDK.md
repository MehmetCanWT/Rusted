# RustedUI — GPUI C# Fluent API, Modder SDK & Theme Isolation Guide

In addition to HTML/Tailwind and React/TSX templates, **RustedUI** provides a programmatic **GPUI-inspired C# Fluent Builder** (`RustedContext` / `RustedNode`) and strict **Per-Mod Theme Isolation** for third-party SMAPI mods.

---

## 1. Building UIs Programmatically with the GPUI C# Fluent Builder

Use `view.BuildWithFluent(ui => ...)` to construct a retained Rust `UiTree` directly from C#:

```csharp
using Rusted.Smapi.UI;

var view = new RustedView(ownerModId: ModManifest.UniqueID);

view.BuildWithFluent(ui =>
    ui.Div("flex flex-col items-center justify-center w-full h-full").Children(
        ui.Div("flex flex-col w-640 h-auto sdv-box gap-3").Id("my_window").Children(
            ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                ui.Banner("Farm Automation Hub", "font-dialogue text-sdv-brown"),
                ui.CloseButton("btn_close")
            ),
            ui.Divider(),
            ui.Panel("flex flex-col gap-3 w-full").Children(
                ui.Text("Watering Pressure: 80%").Id("lbl_pressure"),
                ui.Slider("sld_pressure", 80f, 100f, "pressure_changed", "w-full"),
                ui.ProgressBar("bar_pressure", 80f, 100f, "w-full h-6"),
                ui.Div("flex flex-row justify-between items-center w-full").Children(
                    ui.Icon("zap", "lucide-md"),
                    ui.Badge("Online", "badge-success"),
                    ui.Button("btn_run", "Run Sprinklers", "run_sprinklers")
                )
            )
        )
    )
);
```

---

## 2. O(1) State Updates & Event Binding (`RustedView`)

Once a `RustedView` is loaded (from HTML/TSX or Fluent C#), you can mutate any element in $O(1)$ time by its `id` without rebuilding the tree:

```csharp
// 1. Bind click and value-change actions
view.OnAction("run_sprinklers", () => {
    Game1.playSound("coin");
    view.SetText("lbl_pressure", "Sprinklers Activated!");
});

view.OnAction("pressure_changed", ev => {
    float val = Math.Clamp(ev.FloatValue, 0f, 100f);
    view.SetProgress("bar_pressure", val, 100f);
    view.SetText("lbl_pressure", $"Watering Pressure: {(int)val}%");
});

// 2. O(1) State Updates
view.SetText("lbl_pressure", "Ready");
view.SetProgress("bar_pressure", 95f, 100f);
view.SetChecked("chk_enabled", true);
view.SetVisible("panel_tab_2", false); // Toggles Display::Flex <-> Display::None
view.SetSlotItem("slot_input", itemIndex: 74, count: 5, quality: 4);
view.SetHearts("heart_meter", filled: 9, max: 10);
view.SetTooltip("btn_run", "Activates all iridium sprinklers on the farm");
view.ScrollBy("log_scroll", deltaY: 40f);
```

### Opening a `RustedView` as an In-Game Menu
```csharp
Game1.activeClickableMenu = new RustedMenu(view);
```

---

## 3. Per-Mod Theme Isolation vs. Opt-In Global Theme

When a player selects a color theme in RustedUI (for example, `JunimoForest` or `CatppuccinMocha`), **all of RustedUI's own menus, HUD overlays, and hover tooltips** use that selected theme.

However, **third-party mods using RustedUI are isolated by default** so their custom UI design is never unexpectedly overridden:

1. **Default Behavior (Isolated)**:
   ```csharp
   // External mod creates a RustedView with its own UniqueID:
   var externalView = new RustedView(ownerModId: "SomeAuthor.CustomMachineMod");
   ```
   - `externalView.UseRustedUiTheme` defaults to `false`.
   - `externalView` uses `PelicanClassic` (or whatever theme `SomeAuthor` explicitly set via `externalView.SetTheme(...)` or `theme="..."` in XML).
   - Calling `externalView.ApplyGlobalRustedTheme(globalTheme)` will **NOT** override `externalView`'s theme.

2. **Opt-In to RustedUI's Global Theme (C# or XML)**:
   If a third-party mod author *wants* their mod's UI to automatically follow the player's chosen RustedUI theme, they can opt in via either:
   - **C# Constructor / Property**:
     ```csharp
     var externalView = new RustedView(
         ownerModId: "SomeAuthor.CustomMachineMod",
         useRustedUiTheme: true
     );
     ```
   - **HTML/XML Root Attribute**:
     ```xml
     <div id="root" use-rusted-theme="true" class="flex flex-col w-640 sdv-box">
     ```
