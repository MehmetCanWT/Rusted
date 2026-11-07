using System;
using System.Diagnostics;
using System.IO;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Input;
using Rusted.Smapi.Native;
using Rusted.Smapi.UI;
using StardewModdingAPI;
using StardewModdingAPI.Events;
using StardewValley;

namespace Rusted.Smapi
{
    public sealed class ModEntry : Mod
    {
        private RustedView? _hudView;
        private RustedView? _mainMenuView;
        private RustedView? _creditsView;
        private RustedView? _featureTestView;
        private ModConfig _config = new();

        private bool _showHud = true;
        private bool _soundEnabled = true;
        private bool _useFluentMode;
        private UiThemeId _activeTheme = UiThemeId.PelicanClassic;
        private int _activeTab = 0;
        private float _demoProgress = 80f;
        private int _demoHearts = 8;
        private string _noteText = "Hello RustedUI!";
        private int _totalEventsHandled;
        private int _lastScrollWheelValue;
        private string _lastSlotMessage = "Hover items for Tooltips, or click a slot to inspect!";

        private readonly Process _currentProcess = Process.GetCurrentProcess();
        private TimeSpan _lastCpuTime;
        private long _lastCpuWallTicks;
        private double _lastCpuPercent;

        public override void Entry(IModHelper helper)
        {
            NativeResolver.Initialize(helper, Monitor);

            _config = helper.ReadConfig<ModConfig>();
            ApplyConfigToRuntimeState();

            helper.Events.GameLoop.GameLaunched += OnGameLaunched;
            helper.Events.GameLoop.UpdateTicked += OnUpdateTicked;
            helper.Events.Display.RenderedHud += OnRenderedHud;
            helper.Events.Display.Rendered += OnRenderedGlobal;
            helper.Events.Input.ButtonPressed += OnButtonPressed;
            helper.Events.Input.ButtonsChanged += OnButtonsChanged;

            Monitor.Log($"RustedUI v0.3 initialized! Open Menu: [{_config.OpenMenuKey}] | Toggle HUD: [{_config.ToggleHudKey}]", LogLevel.Info);
        }

        private void ApplyConfigToRuntimeState()
        {
            _showHud = _config.ShowHud;
            _soundEnabled = _config.EnableUiSounds;
            _activeTheme = _config.DefaultTheme switch
            {
                "ShadcnZincDark" => UiThemeId.ShadcnZincDark,
                "WizardArcane" => UiThemeId.WizardArcane,
                "JojaCorp" => UiThemeId.JojaCorp,
                "JunimoForest" => UiThemeId.JunimoForest,
                "CatppuccinMocha" => UiThemeId.CatppuccinMocha,
                "TokyoNight" => UiThemeId.TokyoNight,
                "Dracula" => UiThemeId.Dracula,
                "Nord" => UiThemeId.Nord,
                "Cyberpunk" => UiThemeId.Cyberpunk,
                _ => UiThemeId.PelicanClassic
            };
        }

        private void SaveRuntimeStateToConfig()
        {
            _config.ShowHud = _showHud;
            _config.EnableUiSounds = _soundEnabled;
            _config.DefaultTheme = _activeTheme switch
            {
                UiThemeId.ShadcnZincDark => "ShadcnZincDark",
                UiThemeId.WizardArcane => "WizardArcane",
                UiThemeId.JojaCorp => "JojaCorp",
                UiThemeId.JunimoForest => "JunimoForest",
                UiThemeId.CatppuccinMocha => "CatppuccinMocha",
                UiThemeId.TokyoNight => "TokyoNight",
                UiThemeId.Dracula => "Dracula",
                UiThemeId.Nord => "Nord",
                UiThemeId.Cyberpunk => "Cyberpunk",
                _ => "PelicanClassic"
            };
            Helper.WriteConfig(_config);
        }

        private void OnGameLaunched(object? sender, GameLaunchedEventArgs e)
        {
            _lastScrollWheelValue = Mouse.GetState().ScrollWheelValue;
            InitializeViews();
            RegisterGenericModConfigMenu();
        }

        private void RegisterGenericModConfigMenu()
        {
            // Optional integration: if Generic Mod Config Menu is not installed, skip silently.
            var configMenu = Helper.ModRegistry.GetApi<IGenericModConfigMenuApi>("spacechase0.GenericModConfigMenu");
            if (configMenu == null)
                return;

            configMenu.Register(
                mod: ModManifest,
                reset: () =>
                {
                    _config = new ModConfig();
                    ApplyConfigToRuntimeState();
                    SyncAllViews();
                },
                save: () =>
                {
                    Helper.WriteConfig(_config);
                    ApplyConfigToRuntimeState();
                    SyncAllViews();
                });

            configMenu.AddSectionTitle(
                mod: ModManifest,
                text: () => "HUD & Keybind Settings",
                tooltip: () => "Configure the RustedUI in-game overlay and keyboard shortcuts.");

            configMenu.AddBoolOption(
                mod: ModManifest,
                name: () => "Show In-Game Mini-HUD",
                tooltip: () => "Show or hide the top-left RustedUI telemetry & quick-access HUD overlay.",
                getValue: () => _config.ShowHud,
                setValue: value =>
                {
                    _config.ShowHud = value;
                    _showHud = value;
                });

            configMenu.AddKeybindList(
                mod: ModManifest,
                name: () => "Toggle Mini-HUD Keybind",
                tooltip: () => "Shortcut key to quickly show or hide the in-game Mini-HUD overlay (Default: F7).",
                getValue: () => _config.ToggleHudKey,
                setValue: value => _config.ToggleHudKey = value);

            configMenu.AddKeybindList(
                mod: ModManifest,
                name: () => "Open RustedUI Menu Keybind",
                tooltip: () => "Shortcut key to open or close the RustedUI Control Panel (Default: F8).",
                getValue: () => _config.OpenMenuKey,
                setValue: value => _config.OpenMenuKey = value);

            configMenu.AddSectionTitle(
                mod: ModManifest,
                text: () => "Appearance & Audio",
                tooltip: () => "Customize RustedUI theme tokens and sound cues.");

            configMenu.AddTextOption(
                mod: ModManifest,
                name: () => "Default UI Color Theme",
                tooltip: () => "Choose between Pelican Classic 9-Slice or modern Shadcn / Fantasy themes.",
                getValue: () => _config.DefaultTheme,
                setValue: value => _config.DefaultTheme = value,
                allowedValues: new[]
                {
                    "PelicanClassic",
                    "ShadcnZincDark",
                    "WizardArcane",
                    "JojaCorp",
                    "JunimoForest",
                    "CatppuccinMocha",
                    "TokyoNight",
                    "Dracula",
                    "Nord",
                    "Cyberpunk"
                },
                formatAllowedValue: value => value switch
                {
                    "ShadcnZincDark" => "Shadcn Zinc Dark (#09090B)",
                    "WizardArcane" => "Wizard Arcane Violet (#0F081D)",
                    "JojaCorp" => "JojaCorp Cyber Blue (#0F172A)",
                    "JunimoForest" => "Junimo Forest Emerald (#14261C)",
                    "CatppuccinMocha" => "Catppuccin Mocha (#1E1E2E)",
                    "TokyoNight" => "Tokyo Night (#1A1B26)",
                    "Dracula" => "Dracula (#282A36)",
                    "Nord" => "Nord Polar (#2E3440)",
                    "Cyberpunk" => "Cyberpunk Neon (#0A0616)",
                    _ => "Pelican Classic (Vanilla 9-Slice)"
                });

            configMenu.AddBoolOption(
                mod: ModManifest,
                name: () => "Enable UI Sound Effects",
                tooltip: () => "Play vanilla Stardew Valley audio cues when hovering or clicking RustedUI widgets.",
                getValue: () => _config.EnableUiSounds,
                setValue: value =>
                {
                    _config.EnableUiSounds = value;
                    _soundEnabled = value;
                });
        }

        private void InitializeViews()
        {
            _hudView?.Dispose();
            _mainMenuView?.Dispose();
            _creditsView?.Dispose();
            _featureTestView?.Dispose();

            // 0. In-Game & Title Screen Mini-HUD
            _hudView = new RustedView();
            string hudPath = Path.Combine(Helper.DirectoryPath, "assets", "ui", "test_hud.html");
            _hudView.LoadMarkupFileWithHotReload(hudPath);
            _hudView.OnAction("open_test_menu", ToggleMainMenu);
            _hudView.OnAction("open_menu", ToggleMainMenu);

            // 1. F8 Main Menu (Has the Credits button)
            _mainMenuView = new RustedView();
            string mainPath = Path.Combine(Helper.DirectoryPath, "assets", "ui", "test_menu.html");
            _mainMenuView.LoadMarkupFileWithHotReload(mainPath);
            RegisterSharedActions(_mainMenuView);

            // 2. Credits Menu (Opened by Credits button; has the Test Menu button)
            _creditsView = new RustedView();
            string creditsPath = Path.Combine(Helper.DirectoryPath, "assets", "ui", "credits_menu.html");
            _creditsView.LoadMarkupFileWithHotReload(creditsPath);
            RegisterSharedActions(_creditsView);

            // 3. Comprehensive Feature Test Lab Menu (Opened by Test button in Credits menu)
            _featureTestView = new RustedView();
            RegisterSharedActions(_featureTestView);
            RebuildFeatureTestView();
        }

        private void RebuildFeatureTestView()
        {
            if (_featureTestView == null)
                return;

            if (!_useFluentMode)
            {
                string testPath = Path.Combine(Helper.DirectoryPath, "assets", "ui", "feature_test_menu.html");
                _featureTestView.LoadMarkupFileWithHotReload(testPath);
            }
            else
            {
                // Built 100% programmatically using the GPUI-inspired C# Fluent Builder showcasing ALL components!
                _featureTestView.BuildWithFluent(ui =>
                    ui.Div("flex flex-col items-center justify-center w-full h-full").Children(
                        ui.Div("flex flex-col w-960 h-auto sdv-box gap-3").Id("test_lab_window").Children(
                            // Header
                            ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                ui.Banner("RustedUI - GPUI C# Lab", "font-dialogue text-sdv-brown"),
                                ui.Div("flex flex-row items-center gap-2").Children(
                                    ui.Tab("tab_all", "Split", _activeTab == 0, "select_tab_all"),
                                    ui.Tab("tab_controls", "Inputs", _activeTab == 1, "tab_1"),
                                    ui.Tab("tab_inventory", "Inventory", _activeTab == 2, "tab_2"),
                                    ui.Button("btn_switch_mode", "TSX Mode", "switch_mode"),
                                    ui.CloseButton("btn_close")
                                )
                            ),
                            // Theme Switcher Bar
                            ui.Panel("flex flex-row items-center justify-between w-full px-3 py-1").Children(
                                ui.Div("flex flex-row items-center gap-2").Children(
                                    ui.Icon("palette", "lucide-md"),
                                    ui.Text("Theme: Pelican Classic").Id("lbl_active_theme"),
                                    ui.Badge("GPUI + Shadcn", "badge-success")
                                ),
                                ui.Div("flex flex-row items-center gap-2").Children(
                                    ui.Button("btn_theme_0", "Pelican", "theme_pelican"),
                                    ui.Button("btn_theme_1", "Zinc Dark", "theme_zinc"),
                                    ui.Button("btn_theme_2", "Arcane", "theme_arcane"),
                                    ui.Button("btn_theme_3", "Joja", "theme_joja"),
                                    ui.Button("btn_theme_4", "Junimo", "theme_junimo")
                                )
                            ),
                            ui.Divider(),
                            // Body
                            ui.Div("flex flex-row gap-4 w-full items-stretch").Children(
                                ui.Panel("flex flex-col justify-between gap-3 flex-1").Id("panel_tab_1").Children(
                                    ui.Div("flex flex-col gap-2 w-full").Children(
                                        ui.Title("1. GPUI Inputs, Slider & Hearts").Id("lbl_section_1"),
                                        ui.Div("flex flex-col gap-1 w-full").Children(
                                            ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                                ui.Text("Draggable Slider & Bar:"),
                                                ui.Text($"{(int)_demoProgress} / 100", "font-small text-sdv-brown").Id("lbl_progress_val")
                                            ),
                                            ui.Slider("slider_demo", _demoProgress, 100f, "slider_changed", "w-full"),
                                            ui.ProgressBar("bar_demo", _demoProgress, 100f, "w-full h-6")
                                        ),
                                        ui.Div("flex flex-col gap-1 w-full mt-1").Children(
                                            ui.Text("Text Input (Click & Type) + Trash Clear:"),
                                            ui.Div("flex flex-row items-center gap-2 w-full").Children(
                                                ui.TextInput("inp_note", _noteText, "focus_note", "flex-1"),
                                                ui.TrashButton("btn_clear_note", "clear_note")
                                            )
                                        ),
                                        ui.Div("flex flex-col gap-1 w-full mt-1").Children(
                                            ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                                ui.Text("Friendship Hearts:"),
                                                ui.Text($"{_demoHearts} / 10 Hearts", "font-small text-sdv-brown").Id("lbl_hearts_val")
                                            ),
                                            ui.Div("flex flex-row items-center justify-between w-full gap-2").Children(
                                                ui.ArrowLeft("btn_heart_minus", "heart_minus"),
                                                ui.HeartRating("test_hearts", _demoHearts, 10),
                                                ui.ArrowRight("btn_heart_plus", "heart_plus")
                                            )
                                        ),
                                        ui.Div("flex flex-row justify-between items-center w-full mt-1 gap-2").Children(
                                            ui.Div("flex flex-row items-center gap-2").Children(
                                                ui.Checkbox("chk_hud", _showHud, "toggle_hud"),
                                                ui.Text("Mini-HUD"),
                                                ui.Checkbox("chk_sound", _soundEnabled, "toggle_sound"),
                                                ui.Text("Sounds")
                                            ),
                                            ui.Div("flex flex-row items-center gap-2").Children(
                                                ui.CoinIcon(),
                                                ui.Button("btn_give_gold", "+500G", "give_gold")
                                            )
                                        )
                                    )
                                ),
                                ui.Panel("flex flex-col justify-between flex-1 gap-3").Id("panel_tab_2").Children(
                                    ui.Div("flex flex-col gap-2 w-full").Children(
                                        ui.Title("2. GPUI Inventory & ScrollView"),
                                        ui.Div("flex flex-row items-center justify-between gap-2 w-full").Children(
                                            ui.ItemSlot("slot_prismatic", 74, 7, 4, "slot_prismatic"),
                                            ui.ItemSlot("slot_starfruit", 268, 24, 2, "slot_starfruit"),
                                            ui.ItemSlot("slot_iridium", 337, 50, 4, "slot_iridium"),
                                            ui.ItemSlot("slot_ancient", 454, 12, 1, "slot_ancient"),
                                            ui.ItemSlot("slot_diamond", 72, 99, 4, "slot_diamond")
                                        ),
                                        ui.Text(_lastSlotMessage, "font-small text-sdv-brown w-full").Id("lbl_slot_info"),
                                        ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                            ui.Text("ScrollView (Mouse Wheel & Arrows):"),
                                            ui.Div("flex flex-row items-center gap-2").Children(
                                                ui.ArrowUp("btn_scroll_up", "scroll_up"),
                                                ui.ArrowDown("btn_scroll_down", "scroll_down")
                                            )
                                        ),
                                        ui.ScrollView("test_scroll", "w-full h-160 sdv-panel gap-1").Children(
                                            ui.Text("[01] Built via GPUI C# Fluent API.").Id("lbl_log_1"),
                                            ui.Text("[02] Stored in Rust SlotMap + TaffyTree.").Id("lbl_log_2"),
                                            ui.Text("[03] Hardware ScissorRectangle clip active.").Id("lbl_log_3"),
                                            ui.Text("[04] Slider, Input, Hearts & Arrows ready.").Id("lbl_log_4"),
                                            ui.Text("[05] Zero C# GC heap allocations!").Id("lbl_log_5"),
                                            ui.Text("[06] Scroll down using your Mouse Wheel.").Id("lbl_log_6"),
                                            ui.Text("[07] Full cross-platform Linux & Proton ABI.").Id("lbl_log_7"),
                                            ui.Text("[08] Click HTML Mode to test Hot-Reload!").Id("lbl_log_8")
                                        )
                                    ),
                                    ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                        ui.Div("flex flex-col flex-1 min-w-0").Children(
                                            ui.Text("Render: 0.080 ms (0 GC)").Id("lbl_telemetry_fps"),
                                            ui.Text($"Cmds: 0 | Events: {_totalEventsHandled}").Id("lbl_telemetry_cmds")
                                        ),
                                        ui.Button("btn_back_credits", "Back to Credits", "open_credits")
                                    )
                                )
                            ),
                            ui.Div("flex flex-row justify-between items-center w-full gap-2").Children(
                                ui.Text("GPUI C# Fluent API Mode Active!", "font-small text-sdv-muted").Id("lbl_footer"),
                                ui.Text("[F8: Main Menu | ESC: Close]", "font-small text-sdv-brown")
                            )
                        )
                    )
                );
            }

            SyncAllViews();
        }

        private void RegisterSharedActions(RustedView view)
        {
            view.OnHotReloaded += () =>
            {
                Game1.playSound("shiny4");
                SyncAllViews();
                view.SetText("lbl_footer", $"Hot-Reload Succeeded! ({DateTime.Now:HH:mm:ss})");
            };

            // Navigation between the 3 Menus
            view.OnAction("open_credits", () =>
            {
                _totalEventsHandled++;
                OpenMenu(_creditsView);
            });

            view.OnAction("open_feature_test", () =>
            {
                _totalEventsHandled++;
                OpenMenu(_featureTestView);
            });

            view.OnAction("back_to_main", () =>
            {
                _totalEventsHandled++;
                OpenMenu(_mainMenuView);
            });

            view.OnAction("open_main_menu", () =>
            {
                _totalEventsHandled++;
                OpenMenu(_mainMenuView);
            });

            // 10 Switchable Color Themes
            view.OnAction("theme_pelican", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.PelicanClassic;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_zinc", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.ShadcnZincDark;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_arcane", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.WizardArcane;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_joja", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.JojaCorp;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_junimo", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.JunimoForest;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_catppuccin", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.CatppuccinMocha;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_tokyonight", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.TokyoNight;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_dracula", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.Dracula;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_nord", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.Nord;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });
            view.OnAction("theme_cyberpunk", () =>
            {
                _totalEventsHandled++;
                _activeTheme = UiThemeId.Cyberpunk;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });

            // Tabs in Feature Test Menu (Split View, Tab 1: Inputs, Tab 2: Inventory)
            Action activateTabAll = () =>
            {
                _totalEventsHandled++;
                _activeTab = 0;
                SyncAllViews();
                view.SetText("lbl_footer", "Active Tab: Split View (Both Inputs & Inventory Panels Visible)");
            };
            view.OnAction("tab_all", activateTabAll);
            view.OnAction("select_tab_all", activateTabAll);

            Action activateTab1 = () =>
            {
                _totalEventsHandled++;
                _activeTab = 1;
                SyncAllViews();
                view.SetText("lbl_footer", "Active Tab: 1. Inputs, Draggable Slider, Text Input & Hearts (Full Width)");
            };
            view.OnAction("tab_1", activateTab1);
            view.OnAction("select_tab_1", activateTab1);

            Action activateTab2 = () =>
            {
                _totalEventsHandled++;
                _activeTab = 2;
                SyncAllViews();
                view.SetText("lbl_footer", "Active Tab: 2. Inventory Slots, Quality Stars, Tooltips & ScrollView (Full Width)");
            };
            view.OnAction("tab_2", activateTab2);
            view.OnAction("select_tab_2", activateTab2);

            // Slider & Progress Bar
            view.OnAction("slider_changed", ev =>
            {
                _totalEventsHandled++;
                _demoProgress = Math.Clamp(ev.FloatValue, 0f, 100f);
                SyncAllViews();
            });

            view.OnAction("energy_down", () =>
            {
                _totalEventsHandled++;
                _demoProgress = Math.Max(0f, _demoProgress - 20f);
                SyncAllViews();
            });

            view.OnAction("energy_up", () =>
            {
                _totalEventsHandled++;
                _demoProgress = Math.Min(100f, _demoProgress + 20f);
                SyncAllViews();
            });

            // Text Input & Trash Button
            Action focusInput = () =>
            {
                _totalEventsHandled++;
                view.SetText("lbl_footer", "Text Input focused! Type letters/numbers on your keyboard or press Backspace.");
            };
            view.OnAction("focus_note", focusInput);
            view.OnAction("focus_search", focusInput);

            Action clearInput = () =>
            {
                _totalEventsHandled++;
                _noteText = "";
                SyncAllViews();
                view.SetText("lbl_footer", "Text Input cleared via Trash Button!");
            };
            view.OnAction("clear_note", clearInput);
            view.OnAction("clear_input", clearInput);

            // Friendship Hearts & Left/Right Arrows
            view.OnAction("heart_minus", () =>
            {
                _totalEventsHandled++;
                _demoHearts = Math.Max(0, _demoHearts - 1);
                SyncAllViews();
            });

            view.OnAction("heart_plus", () =>
            {
                _totalEventsHandled++;
                _demoHearts = Math.Min(10, _demoHearts + 1);
                SyncAllViews();
            });

            // ScrollView Up/Down Arrows
            view.OnAction("scroll_up", () =>
            {
                _totalEventsHandled++;
                view.ScrollBy("test_scroll", -40f);
                view.ScrollBy("log_scroll", -40f);
            });

            view.OnAction("scroll_down", () =>
            {
                _totalEventsHandled++;
                view.ScrollBy("test_scroll", 40f);
                view.ScrollBy("log_scroll", 40f);
            });

            // Gold & Inventory Slots
            view.OnAction("give_gold", () =>
            {
                _totalEventsHandled++;
                Game1.playSound("coin");
                if (Context.IsWorldReady && Game1.player != null)
                {
                    Game1.player.Money += 500;
                    _lastSlotMessage = $"Added +500G! Total Gold: {Game1.player.Money}G";
                }
                else
                {
                    _lastSlotMessage = "Simulated +500G on Title Screen! (Adds real gold in-game)";
                }
                SyncAllViews();
            });

            view.OnAction("slot_prismatic", () =>
            {
                _totalEventsHandled++;
                _lastSlotMessage = "Selected: Prismatic Shard (ID: 74, Iridium Quality, Qty: 7)";
                SyncAllViews();
            });

            view.OnAction("slot_starfruit", () =>
            {
                _totalEventsHandled++;
                _lastSlotMessage = "Selected: Starfruit (ID: 268, Gold Quality, Qty: 24)";
                SyncAllViews();
            });

            view.OnAction("slot_iridium", () =>
            {
                _totalEventsHandled++;
                _lastSlotMessage = "Selected: Iridium Bar (ID: 337, Iridium Quality, Qty: 50)";
                SyncAllViews();
            });

            view.OnAction("slot_ancient", () =>
            {
                _totalEventsHandled++;
                _lastSlotMessage = "Selected: Ancient Fruit (ID: 454, Silver Quality, Qty: 12)";
                SyncAllViews();
            });

            view.OnAction("slot_diamond", () =>
            {
                _totalEventsHandled++;
                _lastSlotMessage = "Selected: Diamond (ID: 72, Iridium Quality, Qty: 99)";
                SyncAllViews();
            });

            // Checkboxes & Mode Switch
            view.OnAction("toggle_hud", ev =>
            {
                _totalEventsHandled++;
                _showHud = ev.IntValue != 0;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });

            view.OnAction("toggle_sound", ev =>
            {
                _totalEventsHandled++;
                _soundEnabled = ev.IntValue != 0;
                SaveRuntimeStateToConfig();
                SyncAllViews();
            });

            Action switchMode = () =>
            {
                _totalEventsHandled++;
                _useFluentMode = !_useFluentMode;
                RebuildFeatureTestView();
            };
            view.OnAction("switch_mode", switchMode);
            view.OnAction("toggle_mode", switchMode);
        }

        private void SyncAllViews()
        {
            string themeLabel = _activeTheme switch
            {
                UiThemeId.ShadcnZincDark => "Theme: Shadcn Zinc Dark",
                UiThemeId.WizardArcane => "Theme: Wizard Arcane",
                UiThemeId.JojaCorp => "Theme: JojaCorp Blue",
                UiThemeId.JunimoForest => "Theme: Junimo Forest",
                UiThemeId.CatppuccinMocha => "Theme: Catppuccin Mocha",
                UiThemeId.TokyoNight => "Theme: Tokyo Night",
                UiThemeId.Dracula => "Theme: Dracula",
                UiThemeId.Nord => "Theme: Nord Polar",
                UiThemeId.Cyberpunk => "Theme: Cyberpunk Neon",
                _ => "Theme: Pelican Classic"
            };

            foreach (var v in new[] { _mainMenuView, _creditsView, _featureTestView })
            {
                if (v == null)
                    continue;

                v.ApplyGlobalRustedTheme(_activeTheme);
                v.SoundEnabled = _soundEnabled;
                v.SetText("lbl_active_theme", themeLabel);
                v.SetProgress("bar_demo", _demoProgress, 100f);
                v.SetProgress("bar_energy", _demoProgress, 100f);
                v.SetProgress("slider_demo", _demoProgress, 100f);
                v.SetProgress("sld_energy", _demoProgress, 100f);
                v.SetText("lbl_progress_val", $"{(int)_demoProgress} / 100");
                v.SetText("lbl_slider_val", $"{(int)_demoProgress} / 100");
                v.SetText("lbl_energy_val", $"{(int)_demoProgress} / 100");
                v.SetHearts("test_hearts", _demoHearts, 10);
                v.SetHearts("heart_meter", _demoHearts, 10);
                v.SetText("lbl_hearts_val", $"{_demoHearts} / 10 Hearts");
                v.SetText("inp_note", _noteText);
                v.SetText("inp_search", _noteText);
                v.SetText("lbl_slot_info", _lastSlotMessage);
                v.SetText("lbl_telemetry_clicks", $"Total Rust Events: {_totalEventsHandled}");
                v.SetChecked("chk_hud", _showHud);
                v.SetChecked("chk_sound", _soundEnabled);
                v.SetChecked("tab_all", _activeTab == 0);
                v.SetChecked("tab_controls", _activeTab == 1);
                v.SetChecked("tab_inventory", _activeTab == 2);
                v.SetChecked("tab_1", _activeTab == 1);
                v.SetChecked("tab_2", _activeTab == 2);
                v.SetVisible("panel_tab_1", _activeTab == 0 || _activeTab == 1);
                v.SetVisible("panel_tab_2", _activeTab == 0 || _activeTab == 2);
            }

            if (_hudView != null)
            {
                _hudView.ApplyGlobalRustedTheme(_activeTheme);
                _hudView.SoundEnabled = _soundEnabled;
                _hudView.SetProgress("hud_bar", _demoProgress, 100f);
                _hudView.SetText("btn_open_menu", "Control Panel");
            }
        }

        private void OpenMenu(RustedView? targetView)
        {
            if (targetView == null)
                return;

            targetView.SyncFonts();
            SyncAllViews();
            Game1.playSound("bigSelect");
            Game1.activeClickableMenu = new RustedMenu(targetView, HandleMenuKeyPress);
        }

        private void ToggleMainMenu()
        {
            if (_mainMenuView == null)
                InitializeViews();

            if (Game1.activeClickableMenu is RustedMenu)
            {
                Game1.playSound("drumkit6");
                Game1.activeClickableMenu = null;
            }
            else
            {
                OpenMenu(_mainMenuView);
            }
        }

        private void HandleMenuKeyPress(Keys key)
        {
            if (Game1.activeClickableMenu is not RustedMenu activeMenu)
                return;

            // Live keyboard typing when inp_note is focused in Feature Test Menu
            if (activeMenu.View == _featureTestView)
            {
                if (key == Keys.Back)
                {
                    if (_noteText.Length > 0)
                    {
                        _noteText = _noteText[..^1];
                        Game1.playSound("tinyWhip");
                        SyncAllViews();
                    }
                    return;
                }

                if (key == Keys.Space && _noteText.Length < 34)
                {
                    _noteText += " ";
                    SyncAllViews();
                    return;
                }

                if (key >= Keys.A && key <= Keys.Z && _noteText.Length < 34)
                {
                    KeyboardState ks = Keyboard.GetState();
                    bool shift = ks.IsKeyDown(Keys.LeftShift) || ks.IsKeyDown(Keys.RightShift);
                    char c = (char)('a' + (key - Keys.A));
                    if (shift)
                        c = char.ToUpperInvariant(c);
                    _noteText += c;
                    Game1.playSound("cowboy_monsterhit");
                    SyncAllViews();
                    return;
                }

                if (key >= Keys.D0 && key <= Keys.D9 && _noteText.Length < 34)
                {
                    char c = (char)('0' + (key - Keys.D0));
                    _noteText += c;
                    Game1.playSound("cowboy_monsterhit");
                    SyncAllViews();
                    return;
                }
            }
        }

        private void OnUpdateTicked(object? sender, UpdateTickedEventArgs e)
        {
            if (_hudView == null)
                return;

            MouseState mouse = Mouse.GetState();
            bool leftDown = mouse.LeftButton == ButtonState.Pressed;
            int currentWheel = mouse.ScrollWheelValue;
            float scrollDelta = currentWheel - _lastScrollWheelValue;
            _lastScrollWheelValue = currentWheel;

            if (Game1.activeClickableMenu is RustedMenu activeMenu)
            {
                RustedView activeView = activeMenu.View;
                float uiX = Game1.getMouseX(ui_scale: true);
                float uiY = Game1.getMouseY(ui_scale: true);
                activeView.UpdateInput(uiX, uiY, leftDown, scrollDelta);

                if (e.IsMultipleOf(15))
                {
                    UpdateTelemetryStrings(out string ramCpuShort, out string ramCpuDetail);
                    activeView.SetText("lbl_telemetry_fps", $"Render: {activeView.LastRenderMilliseconds:F3} ms (0 Bytes GC)");
                    activeView.SetText("lbl_render_time", $"Render Time: {activeView.LastRenderMilliseconds:F3} ms (0 Bytes GC)");
                    activeView.SetText("lbl_telemetry_cmds", $"Cmds: {activeView.LastCommandCount} | Events: {_totalEventsHandled}");
                    activeView.SetText("lbl_cmd_count", $"Draw Commands: {activeView.LastCommandCount} | Events: {_totalEventsHandled}");
                    activeView.SetText("lbl_mod_ram_cpu", ramCpuShort);
                    activeView.SetText("lbl_ram_usage", ramCpuDetail);
                }
            }
            else if (_showHud)
            {
                float uiX = Game1.getMouseX(ui_scale: true);
                float uiY = Game1.getMouseY(ui_scale: true);
                _hudView.UpdateInput(uiX, uiY, leftDown, 0f);

                if (e.IsMultipleOf(30))
                {
                    _hudView.SetText("hud_fps", $"{_hudView.LastRenderMilliseconds:F2}ms");
                    _hudView.SetText("hud_perf", $"{_hudView.LastRenderMilliseconds:F2}ms");
                }
            }
        }

        private void UpdateTelemetryStrings(out string ramCpuShort, out string ramCpuDetail)
        {
            long nowTicks = Stopwatch.GetTimestamp();
            TimeSpan cpuNow = _currentProcess.TotalProcessorTime;
            if (_lastCpuWallTicks > 0)
            {
                double wallMs = (nowTicks - _lastCpuWallTicks) * 1000.0 / Stopwatch.Frequency;
                if (wallMs > 50.0)
                {
                    double cpuMs = (cpuNow - _lastCpuTime).TotalMilliseconds;
                    _lastCpuPercent = Math.Clamp(cpuMs / (wallMs * Math.Max(1, Environment.ProcessorCount)) * 100.0, 0.01, 100.0);
                    _lastCpuTime = cpuNow;
                    _lastCpuWallTicks = nowTicks;
                }
            }
            else
            {
                _lastCpuTime = cpuNow;
                _lastCpuWallTicks = nowTicks;
                _lastCpuPercent = 0.08;
            }

            ulong rustBytes = (_hudView?.NativeMemoryBytes ?? 0)
                            + (_mainMenuView?.NativeMemoryBytes ?? 0)
                            + (_creditsView?.NativeMemoryBytes ?? 0)
                            + (_featureTestView?.NativeMemoryBytes ?? 0);
            double rustKb = rustBytes / 1024.0;
            double managedMb = GC.GetTotalMemory(false) / (1024.0 * 1024.0);

            ramCpuShort = $"CPU: {_lastCpuPercent:F1}% | Rust RAM: {rustKb:F1} KB";
            ramCpuDetail = $"CPU: {_lastCpuPercent:F1}% | Rust UI: {rustKb:F1} KB | .NET Heap: {managedMb:F1} MB";
        }

        private void OnButtonsChanged(object? sender, ButtonsChangedEventArgs e)
        {
            if (_config.OpenMenuKey.JustPressed())
            {
                Helper.Input.SuppressActiveKeybinds(_config.OpenMenuKey);
                ToggleMainMenu();
                return;
            }

            if (_config.ToggleHudKey.JustPressed())
            {
                Helper.Input.SuppressActiveKeybinds(_config.ToggleHudKey);
                _showHud = !_showHud;
                SaveRuntimeStateToConfig();
                SyncAllViews();
                if (_soundEnabled)
                {
                    Game1.playSound("smallSelect");
                }
            }
        }

        private void OnButtonPressed(object? sender, ButtonPressedEventArgs e)
        {
            // Suppress world clicks if cursor is over Rusted HUD
            if (e.Button == SButton.MouseLeft && Game1.activeClickableMenu == null && _showHud && _hudView != null)
            {
                float uiX = Game1.getMouseX(ui_scale: true);
                float uiY = Game1.getMouseY(ui_scale: true);
                if (_hudView.UpdateInput(uiX, uiY, true, 0f))
                {
                    Helper.Input.Suppress(SButton.MouseLeft);
                }
            }
        }

        private void OnRenderedHud(object? sender, RenderedHudEventArgs e)
        {
            if (_showHud && Game1.activeClickableMenu == null && _hudView != null)
            {
                _hudView.Draw(e.SpriteBatch);
            }
        }

        private void OnRenderedGlobal(object? sender, RenderedEventArgs e)
        {
            if (Game1.gameMode == Game1.titleScreenGameMode && _showHud && !(Game1.activeClickableMenu is RustedMenu) && _hudView != null)
            {
                _hudView.Draw(e.SpriteBatch);
            }
        }
    }
}
