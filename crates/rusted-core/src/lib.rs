pub mod builder;
pub mod dom;
pub mod font;
pub mod icons;
pub mod tailwind;
pub mod theme;
pub mod types;

pub use builder::DivBuilder;
pub use dom::{ElementNode, NodeKey, UiTree};
pub use font::FontAtlas;
pub use icons::resolve_lucide_icon_rect;
pub use theme::ThemeTokens;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_tailwind_template_and_layout() {
        let mut tree = UiTree::new();
        let markup = r#"
        <div id="root_menu" class="flex flex-col w-640 h-480 sdv-box gap-3">
            <div class="flex flex-row justify-between items-center w-full">
                <banner class="sdv-parchment">
                    <span class="font-dialogue text-sdv-brown">RustedUI Test Menüsü</span>
                </banner>
                <close-button id="btn_close" on:click="close_menu" />
            </div>
            <hr />
            <panel class="flex flex-row items-center gap-4 sdv-panel w-full">
                <item-slot id="slot_1" item="74" count="5" quality="iridium" on:click="inspect_prismatic" />
                <div class="flex flex-col flex-1 gap-2">
                    <span id="lbl_status" class="font-small text-sdv-dark">Prizmatik Parça Analizi</span>
                    <progress-bar id="bar_energy" value="75" max="100" class="w-full h-6" />
                </div>
            </panel>
            <div class="flex flex-row justify-end gap-3 w-full">
                <checkbox id="chk_hud" checked="true" on:click="toggle_hud" />
                <button id="btn_boost" class="sdv-btn" on:click="boost_energy">Enerji Doldur</button>
            </div>
        </div>
        "#;

        let root = tree.load_template_markup(markup).expect("Template should compile");
        assert_eq!(tree.root_key, Some(root));

        let view = tree.compute_and_render(1920.0, 1080.0);
        assert!(view.commands_len >= 10, "Should emit 9-slice, sprite, rect, and text commands");

        // Verify O(1) state updates
        assert!(tree.set_text_by_id("lbl_status", "Güncellendi!"));
        assert!(tree.set_progress_by_id("bar_energy", 100.0, 100.0));
    }

    #[test]
    fn test_all_pelican_components_and_scroll_and_tooltips() {
        let mut tree = UiTree::new();
        let markup = r#"
        <div class="flex flex-col w-900 h-auto sdv-box gap-3">
            <div class="flex flex-row gap-2">
                <tab id="t1" active="true">Sekme 1</tab>
                <tab id="t2">Sekme 2</tab>
            </div>
            <slider id="s1" value="50" max="100" class="w-full" tooltip="Slider ipucu" />
            <text-input id="inp1" class="w-full">Deneme</text-input>
            <trash-button id="tr1" />
            <div class="flex flex-row gap-2">
                <arrow-left id="al" />
                <heart-rating id="hr1" hearts="7" max-hearts="10" />
                <arrow-right id="ar" />
                <arrow-up id="au" />
                <arrow-down id="ad" />
                <coin-icon />
            </div>
            <scroll-view id="sv1" class="w-full h-150 sdv-panel">
                <span>Satır 1</span>
                <span>Satır 2</span>
                <span>Satır 3</span>
                <span>Satır 4</span>
                <span>Satır 5</span>
            </scroll-view>
        </div>
        "#;

        let root = tree.load_template_markup(markup).expect("All components should parse");
        assert_eq!(tree.root_key, Some(root));
        assert!(tree.set_hearts_by_id("hr1", 9, 10));
        assert!(tree.set_tooltip_by_id("s1", "Yeni ipucu"));

        let view = tree.compute_and_render(1920.0, 1080.0);
        assert!(view.commands_len >= 25);
    }

    #[test]
    fn test_shadcn_themes_and_lucide_icons() {
        let mut tree = UiTree::new();
        tree.set_theme(1); // ShadcnZincDark
        let markup = r#"
        <div class="flex flex-col w-900 h-auto sdv-box gap-3">
            <card class="flex flex-row items-center gap-2">
                <icon name="sparkles" class="lucide-md" />
                <badge variant="success">Shadcn Active</badge>
                <switch id="sw_theme" checked="true" />
            </card>
        </div>
        "#;
        let root = tree.load_template_markup(markup).expect("Shadcn & Lucide should parse");
        assert_eq!(tree.root_key, Some(root));
        let view = tree.compute_and_render(1920.0, 1080.0);
        assert!(view.commands_len >= 8);
    }

    #[test]
    fn test_hud_hover_tooltip_uses_selected_theme() {
        let mut tree = UiTree::new();
        let hud_markup = include_str!("../../../src/Rusted.Smapi/assets/ui/test_hud.html");
        tree.load_template_markup(hud_markup).expect("HUD should parse");
        tree.set_theme(4); // JunimoForest

        // First render to compute layout rects
        let _ = tree.compute_and_render(1920.0, 1080.0);
        // Simulate hovering over the HUD card (x=50, y=50)
        let over_ui = tree.update_input(50.0, 50.0, false, 0.0);
        assert!(over_ui, "Mouse at (50, 50) should hit the HUD card");
        let _ = tree.compute_and_render(1920.0, 1080.0);

        // In JunimoForest theme (use_vanilla_9slice == false), NO DrawNineSlice commands should be emitted at all,
        // including for the floating tooltip!
        let has_nine_slice = tree
            .render_commands
            .iter()
            .any(|cmd| cmd.kind == RenderCommandKind::DrawNineSlice as u32);
        assert!(
            !has_nine_slice,
            "Hovering HUD in JunimoForest theme must render a themed tooltip, not a vanilla 9-slice!"
        );
    }

    #[test]
    fn test_all_production_templates_parse_and_fit() {
        let templates = [
            ("test_hud.html", include_str!("../../../src/Rusted.Smapi/assets/ui/test_hud.html")),
            ("test_menu.html", include_str!("../../../src/Rusted.Smapi/assets/ui/test_menu.html")),
            ("credits_menu.html", include_str!("../../../src/Rusted.Smapi/assets/ui/credits_menu.html")),
            ("feature_test_menu.html", include_str!("../../../src/Rusted.Smapi/assets/ui/feature_test_menu.html")),
        ];

        let mut all_overflows = Vec::new();
        for (name, markup) in templates {
            let mut tree = UiTree::new();
            let root = tree
                .load_template_markup(markup)
                .unwrap_or_else(|e| panic!("Failed to parse production template {name}: {e}"));
            assert_eq!(tree.root_key, Some(root));
            let view = tree.compute_and_render(1920.0, 1080.0);
            assert!(view.commands_len >= 5, "{name} should emit render commands");

            // Verify no child horizontally overflows its parent container
            for (_key, node) in &tree.arena {
                if let Some(parent_key) = node.parent {
                    let parent = &tree.arena[parent_key];
                    let child_right = node.computed_rect.x + node.computed_rect.width;
                    let parent_right = parent.computed_rect.x + parent.computed_rect.width;
                    if child_right > parent_right + 2.0 {
                        all_overflows.push(format!(
                            "  - {name}: <{}> (id={:?}, text={:?}) right={child_right} > parent <{}> (id={:?}) right={parent_right} (rect={:?}, parent_rect={:?})",
                            node.tag,
                            node.element_id,
                            node.text_content,
                            parent.tag,
                            parent.element_id,
                            node.computed_rect,
                            parent.computed_rect
                        ));
                    }
                }
            }
        }
        if !all_overflows.is_empty() {
            panic!("Horizontal overflows found:\n{}", all_overflows.join("\n"));
        }
    }
}
