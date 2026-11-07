use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use slotmap::{new_key_type, Key, KeyData, SlotMap};
use taffy::prelude::*;

use crate::font::FontAtlas;
use crate::icons::resolve_lucide_icon_rect;
use crate::tailwind::{apply_tailwind_classes, VisualProps};
use crate::theme::ThemeTokens;
use crate::types::{
    BorderInsets, ColorRgba, ComponentFlags, DirtyFlags, EventSliceView, NativeGlyphMetric,
    RectF, RectI, RenderCommand, RenderCommandKind, RenderSliceView, SoundCueId, TextureId,
    UiEvent, UiEventKind, UiThemeId, VanillaFont,
};

new_key_type! {
    pub struct NodeKey;
}

impl NodeKey {
    pub fn to_bits(self) -> u64 {
        self.data().as_ffi()
    }

    pub fn from_bits(bits: u64) -> Self {
        Self::from(KeyData::from_ffi(bits))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UiNodeContext {
    pub node_key: NodeKey,
    pub is_text: bool,
}

#[derive(Clone, Debug)]
pub struct ElementNode {
    pub key: NodeKey,
    pub taffy_id: NodeId,
    pub tag: String,
    pub element_id: Option<String>,
    pub action_name: Option<String>,
    pub tooltip: Option<String>,
    pub parent: Option<NodeKey>,
    pub children: Vec<NodeKey>,
    pub dirty: DirtyFlags,
    pub flags: ComponentFlags,
    pub style: Style,
    pub visual: VisualProps,
    pub text_content: Option<String>,
    pub computed_rect: RectF,
}

pub struct UiTree {
    pub arena: SlotMap<NodeKey, ElementNode>,
    pub id_lookup: HashMap<String, NodeKey>,
    pub taffy: TaffyTree<UiNodeContext>,
    pub root_key: Option<NodeKey>,
    pub fonts: [FontAtlas; 3],
    pub pixel_zoom: f32,
    pub viewport_size: (f32, f32),
    pub theme: UiThemeId,
    pub use_global_theme: bool,

    // Zero-allocation reusable frame buffers
    pub render_commands: Vec<RenderCommand>,
    pub render_text_pool: Vec<u8>,
    pub events: Vec<UiEvent>,
    pub event_text_pool: Vec<u8>,

    // Input & focus tracking
    pub hovered_node: Option<NodeKey>,
    pub active_node: Option<NodeKey>,
    pub focused_node: Option<NodeKey>,
    pub mouse_was_down: bool,
    pub last_mouse_pos: (f32, f32),
}

impl Default for UiTree {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTree {
    pub fn new() -> Self {
        Self {
            arena: SlotMap::with_key(),
            id_lookup: HashMap::new(),
            taffy: TaffyTree::new(),
            root_key: None,
            fonts: [
                FontAtlas::new_default(VanillaFont::Small),
                FontAtlas::new_default(VanillaFont::Dialogue),
                FontAtlas::new_default(VanillaFont::Tiny),
            ],
            pixel_zoom: 4.0,
            viewport_size: (1920.0, 1080.0),
            theme: UiThemeId::PelicanClassic,
            use_global_theme: false,
            render_commands: Vec::with_capacity(512),
            render_text_pool: Vec::with_capacity(4096),
            events: Vec::with_capacity(64),
            event_text_pool: Vec::with_capacity(512),
            hovered_node: None,
            active_node: None,
            focused_node: None,
            mouse_was_down: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }

    pub fn clear_tree(&mut self) {
        self.arena.clear();
        self.id_lookup.clear();
        self.taffy = TaffyTree::new();
        self.root_key = None;
        self.hovered_node = None;
        self.active_node = None;
        self.focused_node = None;
    }

    pub fn set_theme(&mut self, theme_id: u32) {
        self.theme = match theme_id {
            1 => UiThemeId::ShadcnZincDark,
            2 => UiThemeId::WizardArcane,
            3 => UiThemeId::JojaCorp,
            4 => UiThemeId::JunimoForest,
            5 => UiThemeId::CatppuccinMocha,
            6 => UiThemeId::TokyoNight,
            7 => UiThemeId::Dracula,
            8 => UiThemeId::Nord,
            9 => UiThemeId::Cyberpunk,
            _ => UiThemeId::PelicanClassic,
        };
    }

    pub fn memory_usage_bytes(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        total += self.arena.capacity() * std::mem::size_of::<ElementNode>();
        total += self.id_lookup.capacity() * (std::mem::size_of::<String>() + std::mem::size_of::<NodeKey>());
        total += self.render_commands.capacity() * std::mem::size_of::<RenderCommand>();
        total += self.render_text_pool.capacity();
        total += self.events.capacity() * std::mem::size_of::<UiEvent>();
        total += self.event_text_pool.capacity();
        for (_k, node) in &self.arena {
            total += node.tag.capacity();
            if let Some(ref s) = node.element_id {
                total += s.capacity();
            }
            if let Some(ref s) = node.action_name {
                total += s.capacity();
            }
            if let Some(ref s) = node.tooltip {
                total += s.capacity();
            }
            if let Some(ref s) = node.text_content {
                total += s.capacity();
            }
            total += node.children.capacity() * std::mem::size_of::<NodeKey>();
        }
        total
    }

    pub fn sync_font(
        &mut self,
        font_id: u32,
        line_spacing: f32,
        spacing: f32,
        glyphs: &[NativeGlyphMetric],
    ) {
        if let Some(atlas) = self.fonts.get_mut(font_id as usize) {
            atlas.sync_from_native(line_spacing, spacing, glyphs);
            if let Some(root) = self.root_key {
                if let Some(node) = self.arena.get(root) {
                    let _ = self.taffy.mark_dirty(node.taffy_id);
                }
            }
        }
    }

    pub fn create_element(&mut self, tag: &str, classes: &str) -> NodeKey {
        let mut style = Style::default();
        let mut flags = ComponentFlags::NONE;
        let mut visual = VisualProps::default();

        // Semantic tag defaults
        match tag {
            "button" | "sdv-button" => apply_tailwind_classes("sdv-btn", &mut style, &mut flags, &mut visual),
            "item-slot" | "sdv-slot" => apply_tailwind_classes("sdv-slot", &mut style, &mut flags, &mut visual),
            "progress-bar" | "sdv-bar" => apply_tailwind_classes("sdv-bar", &mut style, &mut flags, &mut visual),
            "checkbox" | "sdv-checkbox" => apply_tailwind_classes("sdv-checkbox", &mut style, &mut flags, &mut visual),
            "close-button" | "sdv-close" => apply_tailwind_classes("sdv-close", &mut style, &mut flags, &mut visual),
            "banner" | "sdv-banner" => apply_tailwind_classes("sdv-parchment", &mut style, &mut flags, &mut visual),
            "panel" | "sdv-panel" => apply_tailwind_classes("sdv-panel", &mut style, &mut flags, &mut visual),
            "hr" | "divider" => apply_tailwind_classes("sdv-divider", &mut style, &mut flags, &mut visual),
            "slider" | "sdv-slider" => apply_tailwind_classes("sdv-slider", &mut style, &mut flags, &mut visual),
            "text-input" | "input" | "sdv-input" => apply_tailwind_classes("sdv-input", &mut style, &mut flags, &mut visual),
            "tab" | "sdv-tab" => apply_tailwind_classes("sdv-tab", &mut style, &mut flags, &mut visual),
            "scroll-view" | "sdv-scroll" => apply_tailwind_classes("sdv-scroll", &mut style, &mut flags, &mut visual),
            "arrow-left" | "sdv-arrow-left" => apply_tailwind_classes("sdv-arrow-left", &mut style, &mut flags, &mut visual),
            "arrow-right" | "sdv-arrow-right" => apply_tailwind_classes("sdv-arrow-right", &mut style, &mut flags, &mut visual),
            "arrow-up" | "sdv-arrow-up" => apply_tailwind_classes("sdv-arrow-up", &mut style, &mut flags, &mut visual),
            "arrow-down" | "sdv-arrow-down" => apply_tailwind_classes("sdv-arrow-down", &mut style, &mut flags, &mut visual),
            "trash-button" | "sdv-trash" => apply_tailwind_classes("sdv-trash", &mut style, &mut flags, &mut visual),
            "heart-rating" | "sdv-hearts" => apply_tailwind_classes("sdv-hearts", &mut style, &mut flags, &mut visual),
            "coin-icon" | "sdv-coin" => apply_tailwind_classes("sdv-coin", &mut style, &mut flags, &mut visual),
            "badge" | "sdv-badge" => apply_tailwind_classes("sdv-badge", &mut style, &mut flags, &mut visual),
            "switch" | "sdv-switch" => apply_tailwind_classes("sdv-switch", &mut style, &mut flags, &mut visual),
            "card" | "shadcn-card" => apply_tailwind_classes("shadcn-card", &mut style, &mut flags, &mut visual),
            "icon" | "lucide-icon" => apply_tailwind_classes("lucide-icon", &mut style, &mut flags, &mut visual),
            "span" | "p" | "label" => {
                style.flex_shrink = 1.0;
                style.min_size.width = Dimension::Length(0.0);
            }
            _ => {}
        }

        if !classes.is_empty() {
            apply_tailwind_classes(classes, &mut style, &mut flags, &mut visual);
        }

        let taffy_id = self
            .taffy
            .new_leaf(style.clone())
            .expect("Failed to allocate Taffy node");

        let key = self.arena.insert(ElementNode {
            key: NodeKey::default(),
            taffy_id,
            tag: tag.to_string(),
            element_id: None,
            action_name: None,
            tooltip: None,
            parent: None,
            children: Vec::new(),
            dirty: DirtyFlags::LAYOUT_DIRTY | DirtyFlags::RENDER_DIRTY,
            flags,
            style,
            visual,
            text_content: None,
            computed_rect: RectF::default(),
        });

        self.arena[key].key = key;
        let _ = self.taffy.set_node_context(
            taffy_id,
            Some(UiNodeContext {
                node_key: key,
                is_text: false,
            }),
        );

        key
    }

    pub fn create_text_node(&mut self, text: &str, classes: &str) -> NodeKey {
        let mut style = Style {
            flex_shrink: 1.0,
            min_size: Size {
                width: Dimension::Length(0.0),
                height: Dimension::Auto,
            },
            ..Style::default()
        };
        let mut flags = ComponentFlags::NONE;
        let mut visual = VisualProps::default();

        if !classes.is_empty() {
            apply_tailwind_classes(classes, &mut style, &mut flags, &mut visual);
        }

        let taffy_id = self
            .taffy
            .new_leaf(style.clone())
            .expect("Failed to allocate Taffy text node");

        let key = self.arena.insert(ElementNode {
            key: NodeKey::default(),
            taffy_id,
            tag: "#text".to_string(),
            element_id: None,
            action_name: None,
            tooltip: None,
            parent: None,
            children: Vec::new(),
            dirty: DirtyFlags::LAYOUT_DIRTY | DirtyFlags::TEXT_MEASURE_DIRTY,
            flags,
            style,
            visual,
            text_content: Some(text.to_string()),
            computed_rect: RectF::default(),
        });

        self.arena[key].key = key;
        let _ = self.taffy.set_node_context(
            taffy_id,
            Some(UiNodeContext {
                node_key: key,
                is_text: true,
            }),
        );

        key
    }

    pub fn append_child(&mut self, parent: NodeKey, child: NodeKey) {
        let parent_taffy = self.arena[parent].taffy_id;
        let child_taffy = self.arena[child].taffy_id;
        let parent_is_scroll = self.arena[parent].flags.contains(ComponentFlags::SDV_SCROLL);
        self.arena[parent].children.push(child);
        self.arena[child].parent = Some(parent);
        if parent_is_scroll {
            self.arena[child].style.flex_shrink = 0.0;
            let _ = self.taffy.set_style(child_taffy, self.arena[child].style.clone());
        }
        let _ = self.taffy.add_child(parent_taffy, child_taffy);
    }

    pub fn set_element_id(&mut self, key: NodeKey, id: &str) {
        if let Some(node) = self.arena.get_mut(key) {
            node.element_id = Some(id.to_string());
            self.id_lookup.insert(id.to_string(), key);
        }
    }

    pub fn find_by_id(&self, id: &str) -> Option<NodeKey> {
        self.id_lookup.get(id).copied()
    }

    /// O(1) text update by element ID or NodeKey.
    pub fn set_text_by_id(&mut self, id: &str, new_text: &str) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        self.set_node_text(key, new_text)
    }

    pub fn set_node_text(&mut self, key: NodeKey, new_text: &str) -> bool {
        let Some(node) = self.arena.get(key) else {
            return false;
        };

        // If node itself is a text node, update it directly; otherwise update its first #text child or create one
        if node.text_content.is_some() || node.tag == "#text" {
            let n = &mut self.arena[key];
            if n.text_content.as_deref() != Some(new_text) {
                n.text_content = Some(new_text.to_string());
                n.dirty |= DirtyFlags::LAYOUT_DIRTY | DirtyFlags::TEXT_MEASURE_DIRTY;
                let _ = self.taffy.mark_dirty(n.taffy_id);
            }
            return true;
        }

        if let Some(&first_child) = node.children.first() {
            if self.arena[first_child].tag == "#text" {
                return self.set_node_text(first_child, new_text);
            }
        }

        let inherited_font = node.visual.font;
        let inherited_color = node.visual.text_color;
        let inherited_flags = node.flags & ComponentFlags::TEXT_SHADOW;
        let text_child = self.create_text_node(new_text, "");
        self.arena[text_child].visual.font = inherited_font;
        self.arena[text_child].visual.text_color = inherited_color;
        self.arena[text_child].flags |= inherited_flags;
        self.append_child(key, text_child);
        true
    }

    /// O(1) progress bar update (RENDER_DIRTY only — does NOT trigger Taffy layout!).
    pub fn set_progress_by_id(&mut self, id: &str, value: f32, max: f32) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            node.visual.progress_value = value.clamp(0.0, max.max(1.0));
            node.visual.progress_max = max.max(1.0);
            node.dirty |= DirtyFlags::RENDER_DIRTY;
            true
        } else {
            false
        }
    }

    pub fn set_checked_by_id(&mut self, id: &str, checked: bool) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            node.flags.set(ComponentFlags::IS_CHECKED, checked);
            node.dirty |= DirtyFlags::RENDER_DIRTY;
            true
        } else {
            false
        }
    }

    pub fn set_visible_by_id(&mut self, id: &str, visible: bool) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            let new_display = if visible { Display::Flex } else { Display::None };
            if node.style.display != new_display {
                node.style.display = new_display;
                node.dirty |= DirtyFlags::LAYOUT_DIRTY | DirtyFlags::RENDER_DIRTY;
                let _ = self.taffy.set_style(node.taffy_id, node.style.clone());
            }
            true
        } else {
            false
        }
    }

    pub fn set_slot_item_by_id(&mut self, id: &str, item_index: i32, count: i32, quality: u8) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            node.visual.item_index = item_index;
            node.visual.item_count = count;
            node.visual.item_quality = quality;
            node.dirty |= DirtyFlags::RENDER_DIRTY;
            true
        } else {
            false
        }
    }

    pub fn set_hearts_by_id(&mut self, id: &str, filled: i32, max: i32) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            let clamped_max = max.clamp(1, 14);
            node.visual.hearts_max = clamped_max;
            node.visual.hearts_filled = filled.clamp(0, clamped_max);
            node.dirty |= DirtyFlags::RENDER_DIRTY;
            true
        } else {
            false
        }
    }

    pub fn set_tooltip_by_id(&mut self, id: &str, tooltip: &str) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            node.tooltip = if tooltip.is_empty() {
                None
            } else {
                Some(tooltip.to_string())
            };
            true
        } else {
            false
        }
    }

    pub fn scroll_by_id(&mut self, id: &str, delta_y: f32) -> bool {
        let Some(key) = self.find_by_id(id) else {
            return false;
        };
        if let Some(node) = self.arena.get_mut(key) {
            let max_s = node.visual.max_scroll_y.max(0.0);
            node.visual.scroll_y = (node.visual.scroll_y + delta_y).clamp(0.0, max_s);
            node.dirty |= DirtyFlags::RENDER_DIRTY;
            true
        } else {
            false
        }
    }

    /// Compiles an HTML / XML template string (browserless) directly into the SlotMap + TaffyTree.
    pub fn load_template_markup(&mut self, markup: &str) -> Result<NodeKey, String> {
        // Preserve existing checked/progress/hearts/scroll state by element_id across Hot-Reloads!
        let mut saved_states: HashMap<String, (bool, f32, f32, i32, i32, u8, i32, i32, f32)> = HashMap::new();
        for (id, &key) in &self.id_lookup {
            if let Some(n) = self.arena.get(key) {
                saved_states.insert(
                    id.clone(),
                    (
                        n.flags.contains(ComponentFlags::IS_CHECKED),
                        n.visual.progress_value,
                        n.visual.progress_max,
                        n.visual.item_index,
                        n.visual.item_count,
                        n.visual.item_quality,
                        n.visual.hearts_filled,
                        n.visual.hearts_max,
                        n.visual.scroll_y,
                    ),
                );
            }
        }

        self.clear_tree();

        let mut reader = Reader::from_str(markup);
        reader.config_mut().trim_text(true);

        let mut stack: Vec<NodeKey> = Vec::new();
        let mut root: Option<NodeKey> = None;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let node_key = self.parse_xml_element(e, &saved_states)?;
                    if let Some(&parent) = stack.last() {
                        self.append_child(parent, node_key);
                    } else if root.is_none() {
                        root = Some(node_key);
                    }
                    stack.push(node_key);
                }
                Ok(Event::Empty(ref e)) => {
                    let node_key = self.parse_xml_element(e, &saved_states)?;
                    if let Some(&parent) = stack.last() {
                        self.append_child(parent, node_key);
                    } else if root.is_none() {
                        root = Some(node_key);
                    }
                }
                Ok(Event::Text(ref e)) => {
                    let decoded = e.unescape().unwrap_or_else(|_| {
                        std::borrow::Cow::Borrowed(
                            std::str::from_utf8(e.as_ref()).unwrap_or_default(),
                        )
                    });
                    let trimmed = decoded.trim();
                    if !trimmed.is_empty() {
                        if let Some(&parent) = stack.last() {
                            let inherited_font = self.arena[parent].visual.font;
                            let inherited_color = self.arena[parent].visual.text_color;
                            let inherited_flags = self.arena[parent].flags & ComponentFlags::TEXT_SHADOW;
                            let text_key = self.create_text_node(trimmed, "");
                            self.arena[text_key].visual.font = inherited_font;
                            self.arena[text_key].visual.text_color = inherited_color;
                            self.arena[text_key].flags |= inherited_flags;
                            self.append_child(parent, text_key);
                        }
                    }
                }
                Ok(Event::End(_)) => {
                    stack.pop();
                }
                Ok(Event::Eof) => break,
                Err(err) => return Err(format!("Template parse error: {err}")),
                _ => {}
            }
        }

        let root_key = root.ok_or_else(|| "Empty template: no root element found".to_string())?;
        self.root_key = Some(root_key);
        Ok(root_key)
    }

    fn parse_xml_element(
        &mut self,
        e: &quick_xml::events::BytesStart<'_>,
        saved_states: &HashMap<String, (bool, f32, f32, i32, i32, u8, i32, i32, f32)>,
    ) -> Result<NodeKey, String> {
        let tag = std::str::from_utf8(e.name().as_ref())
            .map_err(|err| err.to_string())?
            .to_string();

        let mut classes = String::new();
        let mut elem_id: Option<String> = None;
        let mut action: Option<String> = None;
        let mut tooltip_attr: Option<String> = None;
        let mut val_attr: Option<f32> = None;
        let mut max_attr: Option<f32> = None;
        let mut checked_attr: Option<bool> = None;
        let mut item_idx_attr: Option<i32> = None;
        let mut item_cnt_attr: Option<i32> = None;
        let mut item_qual_attr: Option<u8> = None;
        let mut hearts_attr: Option<i32> = None;
        let mut max_hearts_attr: Option<i32> = None;
        let mut icon_attr: Option<String> = None;
        let mut variant_attr: Option<u8> = None;

        for attr in e.attributes().flatten() {
            let k = std::str::from_utf8(attr.key.as_ref()).unwrap_or_default();
            let v_cow = attr.unescape_value().unwrap_or_else(|_| {
                std::borrow::Cow::Borrowed(std::str::from_utf8(attr.value.as_ref()).unwrap_or_default())
            });
            let v = v_cow.as_ref();
            match k {
                "class" => classes = v.to_string(),
                "id" => elem_id = Some(v.to_string()),
                "on:click" | "onclick" | "action" => action = Some(v.to_string()),
                "tooltip" | "title" => tooltip_attr = Some(v.to_string()),
                "value" => val_attr = v.parse::<f32>().ok(),
                "max" => max_attr = v.parse::<f32>().ok(),
                "checked" | "active" => checked_attr = Some(v == "true" || v == "1" || v == "checked" || v == "active"),
                "item" | "item-index" => item_idx_attr = v.parse::<i32>().ok(),
                "count" => item_cnt_attr = v.parse::<i32>().ok(),
                "hearts" => hearts_attr = v.parse::<i32>().ok(),
                "max-hearts" => max_hearts_attr = v.parse::<i32>().ok(),
                "icon" | "name" => icon_attr = Some(v.to_string()),
                "use-rusted-theme" => {
                    self.use_global_theme = v == "true" || v == "1";
                }
                "theme" => {
                    let parsed_theme = match v.to_ascii_lowercase().as_str() {
                        "zinc" | "shadcnzincdark" => UiThemeId::ShadcnZincDark,
                        "arcane" | "wizardarcane" => UiThemeId::WizardArcane,
                        "joja" | "jojacorp" => UiThemeId::JojaCorp,
                        "junimo" | "junimoforest" => UiThemeId::JunimoForest,
                        "catppuccin" | "catppuccinmocha" => UiThemeId::CatppuccinMocha,
                        "tokyonight" | "tokyo" => UiThemeId::TokyoNight,
                        "dracula" => UiThemeId::Dracula,
                        "nord" => UiThemeId::Nord,
                        "cyberpunk" => UiThemeId::Cyberpunk,
                        _ => UiThemeId::PelicanClassic,
                    };
                    self.theme = parsed_theme;
                }
                "variant" => {
                    variant_attr = Some(match v {
                        "secondary" => 1,
                        "outline" => 2,
                        "destructive" | "danger" => 3,
                        "success" => 4,
                        "ghost" => 5,
                        _ => 0,
                    });
                }
                "quality" => {
                    item_qual_attr = Some(match v {
                        "silver" | "1" => 1,
                        "gold" | "2" => 2,
                        "iridium" | "4" => 4,
                        _ => 0,
                    });
                }
                _ => {}
            }
        }

        let key = self.create_element(&tag, &classes);

        if let Some(a) = action {
            self.arena[key].action_name = Some(a);
        }
        if let Some(t) = tooltip_attr {
            self.arena[key].tooltip = Some(t);
        }
        if let Some(v) = val_attr {
            self.arena[key].visual.progress_value = v;
        }
        if let Some(m) = max_attr {
            self.arena[key].visual.progress_max = m.max(1.0);
        }
        if let Some(c) = checked_attr {
            self.arena[key].flags.set(ComponentFlags::IS_CHECKED, c);
        }
        if let Some(idx) = item_idx_attr {
            self.arena[key].visual.item_index = idx;
        }
        if let Some(cnt) = item_cnt_attr {
            self.arena[key].visual.item_count = cnt;
        }
        if let Some(q) = item_qual_attr {
            self.arena[key].visual.item_quality = q;
        }
        if let Some(mh) = max_hearts_attr {
            self.arena[key].visual.hearts_max = mh.clamp(1, 14);
        }
        if let Some(h) = hearts_attr {
            let mh = self.arena[key].visual.hearts_max;
            self.arena[key].visual.hearts_filled = h.clamp(0, mh);
        }
        if let Some(ic) = icon_attr {
            self.arena[key].visual.icon_name = Some(ic);
            self.arena[key].flags.insert(ComponentFlags::LUCIDE_ICON);
        }
        if let Some(var) = variant_attr {
            self.arena[key].visual.variant = var;
        }

        if let Some(id) = elem_id {
            if let Some(&(chk, pval, pmax, iidx, icnt, iqual, hfill, hmax, scrolly)) = saved_states.get(&id) {
                self.arena[key].flags.set(ComponentFlags::IS_CHECKED, chk);
                self.arena[key].visual.progress_value = pval;
                self.arena[key].visual.progress_max = pmax;
                self.arena[key].visual.hearts_filled = hfill;
                self.arena[key].visual.hearts_max = hmax;
                self.arena[key].visual.scroll_y = scrolly;
                if iidx >= 0 {
                    self.arena[key].visual.item_index = iidx;
                    self.arena[key].visual.item_count = icnt;
                    self.arena[key].visual.item_quality = iqual;
                }
            }
            self.set_element_id(key, &id);
        }

        Ok(key)
    }

    /// Processes mouse position, button state, and scroll wheel delta, generating Pull-Based `UiEvent`s with zero callbacks.
    pub fn update_input(&mut self, mouse_x: f32, mouse_y: f32, mouse_down: bool, scroll_delta: f32) -> bool {
        self.events.clear();
        self.event_text_pool.clear();
        self.last_mouse_pos = (mouse_x, mouse_y);

        let Some(root_key) = self.root_key else {
            self.mouse_was_down = mouse_down;
            return false;
        };

        let hit_node = self.hit_test_recursive(root_key, mouse_x, mouse_y);
        let is_over_any_ui = hit_node.is_some();

        // Scroll wheel handling on nearest SDV_SCROLL ancestor
        if scroll_delta.abs() > 0.01 {
            let mut cur = hit_node;
            while let Some(k) = cur {
                if let Some(n) = self.arena.get_mut(k) {
                    if n.flags.contains(ComponentFlags::SDV_SCROLL) {
                        let max_s = n.visual.max_scroll_y.max(0.0);
                        n.visual.scroll_y = (n.visual.scroll_y - scroll_delta * 0.4).clamp(0.0, max_s);
                        break;
                    }
                    cur = n.parent;
                } else {
                    break;
                }
            }
        }

        // Hover transitions
        if self.hovered_node != hit_node {
            if let Some(prev) = self.hovered_node {
                if let Some(n) = self.arena.get_mut(prev) {
                    n.flags.remove(ComponentFlags::IS_HOVERED);
                }
                self.push_event(prev, UiEventKind::HoverLeave, SoundCueId::None, mouse_x, mouse_y, 0.0, 0);
            }

            if let Some(curr) = hit_node {
                let sound = if let Some(n) = self.arena.get_mut(curr) {
                    n.flags.insert(ComponentFlags::IS_HOVERED);
                    if n.is_interactive() {
                        SoundCueId::Shiny4
                    } else {
                        SoundCueId::None
                    }
                } else {
                    SoundCueId::None
                };
                self.push_event(curr, UiEventKind::HoverEnter, sound, mouse_x, mouse_y, 0.0, 0);
            }

            self.hovered_node = hit_node;
        }

        // Click / Active transitions & Slider continuous dragging
        if mouse_down && !self.mouse_was_down {
            self.active_node = hit_node;
            // Clear focus from previous input if clicking elsewhere
            if let Some(prev_focus) = self.focused_node {
                if Some(prev_focus) != hit_node {
                    if let Some(nf) = self.arena.get_mut(prev_focus) {
                        nf.flags.remove(ComponentFlags::IS_FOCUSED);
                    }
                    self.focused_node = None;
                }
            }
            if let Some(active) = hit_node {
                let is_slider = if let Some(n) = self.arena.get_mut(active) {
                    n.flags.insert(ComponentFlags::IS_ACTIVE);
                    n.flags.contains(ComponentFlags::SDV_SLIDER)
                } else {
                    false
                };
                if is_slider {
                    self.update_slider_from_mouse(active, mouse_x, mouse_y, true);
                }
            }
        } else if mouse_down && self.mouse_was_down {
            // Dragging active slider
            if let Some(active) = self.active_node {
                let is_slider = self.arena.get(active).map_or(false, |n| n.flags.contains(ComponentFlags::SDV_SLIDER));
                if is_slider {
                    self.update_slider_from_mouse(active, mouse_x, mouse_y, false);
                }
            }
        } else if !mouse_down && self.mouse_was_down {
            if let Some(active) = self.active_node.take() {
                let is_slider = if let Some(n) = self.arena.get_mut(active) {
                    n.flags.remove(ComponentFlags::IS_ACTIVE);
                    n.flags.contains(ComponentFlags::SDV_SLIDER)
                } else {
                    false
                };
                if !is_slider && hit_node == Some(active) {
                    self.handle_node_activation(active, mouse_x, mouse_y);
                }
            }
        }

        self.mouse_was_down = mouse_down;
        is_over_any_ui
    }

    fn update_slider_from_mouse(&mut self, key: NodeKey, mouse_x: f32, mouse_y: f32, play_sound: bool) {
        let Some(node) = self.arena.get_mut(key) else { return };
        let rect = node.computed_rect;
        if rect.width <= 1.0 {
            return;
        }
        let ratio = ((mouse_x - rect.x) / rect.width).clamp(0.0, 1.0);
        let max_val = node.visual.progress_max.max(1.0);
        let new_val = (ratio * max_val).round();
        if (new_val - node.visual.progress_value).abs() >= 0.5 || play_sound {
            node.visual.progress_value = new_val;
            let int_v = new_val as i32;
            let sound = if play_sound { SoundCueId::SmallSelect } else { SoundCueId::None };
            self.push_event(key, UiEventKind::ValueChanged, sound, mouse_x, mouse_y, new_val, int_v);
        }
    }

    fn handle_node_activation(&mut self, key: NodeKey, mouse_x: f32, mouse_y: f32) {
        let Some(node) = self.arena.get_mut(key) else {
            return;
        };

        let flags = node.flags;
        let item_index = node.visual.item_index;
        let interactive = node.is_interactive();

        if flags.intersects(ComponentFlags::SDV_CHECKBOX | ComponentFlags::SHADCN_SWITCH) {
            let new_checked = !flags.contains(ComponentFlags::IS_CHECKED);
            node.flags.set(ComponentFlags::IS_CHECKED, new_checked);
            let val_f = if new_checked { 1.0 } else { 0.0 };
            let val_i = if new_checked { 1 } else { 0 };
            self.push_event(key, UiEventKind::ValueChanged, SoundCueId::SmallSelect, mouse_x, mouse_y, val_f, val_i);
            self.push_event(key, UiEventKind::Click, SoundCueId::None, mouse_x, mouse_y, val_f, val_i);
        } else if flags.contains(ComponentFlags::SDV_INPUT) {
            node.flags.insert(ComponentFlags::IS_FOCUSED);
            self.focused_node = Some(key);
            self.push_event(key, UiEventKind::FocusInput, SoundCueId::SmallSelect, mouse_x, mouse_y, 0.0, 0);
            self.push_event(key, UiEventKind::Click, SoundCueId::None, mouse_x, mouse_y, 0.0, 0);
        } else if flags.contains(ComponentFlags::SDV_TRASH) {
            self.push_event(key, UiEventKind::Click, SoundCueId::Trashcan, mouse_x, mouse_y, 0.0, 0);
        } else if flags.contains(ComponentFlags::SDV_TAB) {
            self.push_event(key, UiEventKind::Click, SoundCueId::SmallSelect, mouse_x, mouse_y, 0.0, 0);
        } else if flags.intersects(
            ComponentFlags::SDV_ARROW_LEFT
                | ComponentFlags::SDV_ARROW_RIGHT
                | ComponentFlags::SDV_ARROW_UP
                | ComponentFlags::SDV_ARROW_DOWN,
        ) {
            self.push_event(key, UiEventKind::Click, SoundCueId::SmallSelect, mouse_x, mouse_y, 0.0, 0);
        } else if flags.contains(ComponentFlags::SDV_CLOSE) {
            self.push_event(key, UiEventKind::CloseRequested, SoundCueId::Drumkit6, mouse_x, mouse_y, 0.0, 0);
        } else if flags.contains(ComponentFlags::SDV_SLOT) {
            self.push_event(key, UiEventKind::Click, SoundCueId::Dwop, mouse_x, mouse_y, 0.0, item_index);
        } else if interactive {
            self.push_event(key, UiEventKind::Click, SoundCueId::BigSelect, mouse_x, mouse_y, 0.0, 0);
        }
    }

    fn push_event(
        &mut self,
        key: NodeKey,
        kind: UiEventKind,
        sound: SoundCueId,
        mouse_x: f32,
        mouse_y: f32,
        float_value: f32,
        int_value: i32,
    ) {
        let (action_offset, action_len) = if let Some(node) = self.arena.get(key) {
            if let Some(ref act) = node.action_name {
                let off = self.event_text_pool.len() as u32;
                let bytes = act.as_bytes();
                self.event_text_pool.extend_from_slice(bytes);
                (off, bytes.len() as u32)
            } else if let Some(ref id) = node.element_id {
                let off = self.event_text_pool.len() as u32;
                let bytes = id.as_bytes();
                self.event_text_pool.extend_from_slice(bytes);
                (off, bytes.len() as u32)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        self.events.push(UiEvent {
            node_key_bits: key.to_bits(),
            kind: kind as u32,
            sound_cue: sound as u32,
            action_offset,
            action_len,
            mouse_x,
            mouse_y,
            float_value,
            int_value,
        });
    }

    fn hit_test_recursive(&self, key: NodeKey, x: f32, y: f32) -> Option<NodeKey> {
        let node = self.arena.get(key)?;
        if node.style.display == Display::None {
            return None;
        }

        // Early pruning: if this container clips its children, do not hit-test off-screen children outside its bounds!
        if node
            .flags
            .intersects(ComponentFlags::CLIP_CHILDREN | ComponentFlags::SDV_SCROLL)
            && !node.computed_rect.contains(x, y)
        {
            return None;
        }

        // Check children in reverse order (topmost drawn last)
        for &child in node.children.iter().rev() {
            if let Some(hit) = self.hit_test_recursive(child, x, y) {
                return Some(hit);
            }
        }

        if node.computed_rect.contains(x, y)
            && (node.is_interactive()
                || node.tooltip.is_some()
                || node.flags.intersects(
                    ComponentFlags::SDV_BOX
                        | ComponentFlags::SDV_PANEL
                        | ComponentFlags::SDV_PARCHMENT
                        | ComponentFlags::SDV_SCROLL,
                ))
        {
            return Some(key);
        }

        None
    }

    /// Computes Taffy layout, snaps coordinates to Stardew's pixel grid, and synthesizes flat RenderCommands.
    pub fn compute_and_render(&mut self, viewport_w: f32, viewport_h: f32) -> RenderSliceView {
        self.viewport_size = (viewport_w, viewport_h);
        self.render_commands.clear();
        self.render_text_pool.clear();

        let Some(root_key) = self.root_key else {
            return self.current_render_view();
        };

        let root_taffy = self.arena[root_key].taffy_id;
        let arena_ref = &self.arena;
        let fonts_ref = &self.fonts;

        self.taffy
            .compute_layout_with_measure(
                root_taffy,
                Size {
                    width: AvailableSpace::Definite(viewport_w),
                    height: AvailableSpace::Definite(viewport_h),
                },
                |known_dimensions, available_space, _node_id, node_context, _style| {
                    if let Some(ctx) = node_context {
                        if ctx.is_text {
                            if let Some(node) = arena_ref.get(ctx.node_key) {
                                if let Some(ref text) = node.text_content {
                                    let atlas = &fonts_ref[node.visual.font as usize];
                                    let max_w = match available_space.width {
                                        AvailableSpace::Definite(w) => w,
                                        AvailableSpace::MaxContent => f32::INFINITY,
                                        AvailableSpace::MinContent => {
                                            atlas.measure_min_content_width(text, node.visual.font_scale)
                                        }
                                    };
                                    let (w, h) = atlas.measure_text(text, max_w, node.visual.font_scale);
                                    return Size {
                                        width: known_dimensions.width.unwrap_or(w),
                                        height: known_dimensions.height.unwrap_or(h),
                                    };
                                }
                            }
                        }
                    }
                    Size {
                        width: known_dimensions.width.unwrap_or(0.0),
                        height: known_dimensions.height.unwrap_or(0.0),
                    }
                },
            )
            .expect("Taffy layout computation succeeded");

        self.snap_and_emit_recursive(root_key, 0.0, 0.0);
        self.emit_floating_tooltip(viewport_w, viewport_h);
        self.current_render_view()
    }

    fn emit_floating_tooltip(&mut self, viewport_w: f32, viewport_h: f32) {
        let mut cur = self.hovered_node;
        let mut tip_text: Option<String> = None;
        while let Some(k) = cur {
            if let Some(n) = self.arena.get(k) {
                if let Some(ref t) = n.tooltip {
                    if !t.is_empty() {
                        tip_text = Some(t.clone());
                        break;
                    }
                }
                cur = n.parent;
            } else {
                break;
            }
        }

        let Some(text) = tip_text else { return };
        let (mx, my) = self.last_mouse_pos;
        let max_tip_w = 340.0;
        let (tw, th) = self.fonts[VanillaFont::Small as usize].measure_text(&text, max_tip_w, 1.0);
        let box_w = (tw + 36.0).max(120.0);
        let box_h = (th + 32.0).max(56.0);
        let tip_x = (mx + 24.0).clamp(8.0, (viewport_w - box_w - 8.0).max(8.0)).round();
        let tip_y = (my + 24.0).clamp(8.0, (viewport_h - box_h - 8.0).max(8.0)).round();
        let tip_rect = RectF::new(tip_x, tip_y, box_w, box_h);
        let theme_tokens = ThemeTokens::for_theme(self.theme);

        if theme_tokens.use_vanilla_9slice {
            // Drop shadow
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(tip_rect.x + 6.0, tip_rect.y + 6.0, tip_rect.width, tip_rect.height),
                color: ColorRgba::SDV_DROP_SHADOW,
                ..Default::default()
            });
            // Tooltip 9-slice frame
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawNineSlice as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: tip_rect,
                source_rect: RectI::new(384, 373, 18, 18),
                insets: BorderInsets::uniform(6.0),
                scale: 4.0,
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        } else {
            // Modern / Theme-aware Tooltip Surface (matches selected HUD / Menu theme)
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(tip_rect.x + 6.0, tip_rect.y + 6.0, tip_rect.width, tip_rect.height),
                color: ColorRgba::new(0, 0, 0, 150),
                ..Default::default()
            });
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: tip_rect,
                color: theme_tokens.border_highlight,
                ..Default::default()
            });
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(tip_rect.x + 3.0, tip_rect.y + 3.0, tip_rect.width - 6.0, tip_rect.height - 6.0),
                color: theme_tokens.card,
                ..Default::default()
            });
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(tip_rect.x + 3.0, tip_rect.y + 3.0, tip_rect.width - 6.0, 3.0),
                color: theme_tokens.primary,
                ..Default::default()
            });
        }

        let text_col = if theme_tokens.use_vanilla_9slice {
            ColorRgba::SDV_TEXT_DARK
        } else {
            theme_tokens.text_main
        };
        let shadow_col = if theme_tokens.use_vanilla_9slice {
            ColorRgba::SDV_TEXT_SHADOW
        } else {
            ColorRgba::new(0, 0, 0, 160)
        };

        let mut lines_buf: Vec<(String, f32)> = Vec::new();
        self.fonts[VanillaFont::Small as usize].for_each_wrapped_line(
            &text,
            max_tip_w + 8.0,
            1.0,
            |line_str, _idx, y_off| {
                lines_buf.push((line_str.to_string(), y_off));
            },
        );
        for (line_str, y_off) in lines_buf {
            let (off, len) = self.push_render_text(&line_str);
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawText as u32,
                font_id: VanillaFont::Small as u32,
                dest_rect: RectF::new(tip_x + 18.0, (tip_y + 16.0 + y_off).round(), tw, th),
                color: text_col,
                shadow_color: shadow_col,
                shadow_offset_x: 2.0,
                shadow_offset_y: 2.0,
                scale: 1.0,
                text_offset: off,
                text_len: len,
                ..Default::default()
            });
        }
    }

    fn snap_and_emit_recursive(&mut self, key: NodeKey, parent_x: f32, parent_y: f32) {
        let Some(node_snapshot) = self.arena.get(key).cloned() else {
            return;
        };
        if node_snapshot.style.display == Display::None {
            return;
        }

        let Ok(layout) = self.taffy.layout(node_snapshot.taffy_id) else {
            return;
        };

        // Snap to 2px / 4px pixel-art boundary to eliminate sub-pixel shimmering
        let abs_x = (parent_x + layout.location.x).round();
        let abs_y = (parent_y + layout.location.y).round();
        let width = layout.size.width.round();
        let height = layout.size.height.round();

        let rect = RectF::new(abs_x, abs_y, width, height);
        if let Some(n) = self.arena.get_mut(key) {
            n.computed_rect = rect;
            n.dirty = DirtyFlags::empty();
        }

        let flags = node_snapshot.flags;
        let theme_tokens = ThemeTokens::for_theme(self.theme);

        // 1. SDV_BOX: Vanilla IClickableMenu.drawTextureBox OR Theme Window Surface
        if flags.contains(ComponentFlags::SDV_BOX) {
            if theme_tokens.use_vanilla_9slice {
                let box_tint = node_snapshot.visual.bg_color.unwrap_or(ColorRgba::WHITE);
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 8.0, rect.y + 8.0, rect.width - 4.0, rect.height - 4.0),
                    color: ColorRgba::SDV_DROP_SHADOW,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(384, 373, 18, 18),
                    insets: BorderInsets::uniform(6.0),
                    scale: 4.0,
                    color: box_tint,
                    ..Default::default()
                });
            } else {
                // Modern / Fantasy Theme Window Surface
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 8.0, rect.y + 8.0, rect.width, rect.height),
                    color: ColorRgba::new(0, 0, 0, 140),
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: theme_tokens.border,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 4.0, rect.y + 4.0, rect.width - 8.0, rect.height - 8.0),
                    color: theme_tokens.background,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 4.0, rect.y + 4.0, rect.width - 8.0, 4.0),
                    color: theme_tokens.border_highlight,
                    ..Default::default()
                });
            }
        }

        // 2. SDV_PANEL / SHADCN_CARD: Inset wooden control container OR Shadcn Card surface
        if flags.intersects(ComponentFlags::SDV_PANEL | ComponentFlags::SHADCN_CARD) {
            if theme_tokens.use_vanilla_9slice && !flags.contains(ComponentFlags::SHADCN_CARD) {
                let panel_tint = node_snapshot.visual.bg_color.unwrap_or(ColorRgba::WHITE);
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::MenuTiles as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(0, 256, 60, 60),
                    insets: BorderInsets::uniform(16.0),
                    scale: 1.0,
                    color: panel_tint,
                    ..Default::default()
                });
            } else {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: theme_tokens.border,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: theme_tokens.card,
                    ..Default::default()
                });
            }
        }

        // 3. SDV_PARCHMENT: Banner scroll background OR Theme Header Bar
        if flags.contains(ComponentFlags::SDV_PARCHMENT) {
            if theme_tokens.use_vanilla_9slice {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(325, 318, 25, 18),
                    insets: BorderInsets::new(4.0, 12.0, 4.0, 12.0),
                    scale: 4.0,
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            } else {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: theme_tokens.border_highlight,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: theme_tokens.muted,
                    ..Default::default()
                });
            }
        }

        // 4. SDV_BUTTON: Interactive button (Vanilla 9-Slice OR Theme Button)
        if flags.contains(ComponentFlags::SDV_BUTTON) {
            let is_hovered = flags.contains(ComponentFlags::IS_HOVERED);
            let is_active = flags.contains(ComponentFlags::IS_ACTIVE);
            let btn_rect = if is_active {
                RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width, rect.height)
            } else {
                rect
            };
            if theme_tokens.use_vanilla_9slice {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 4.0, rect.y + 4.0, rect.width, rect.height),
                    color: ColorRgba::SDV_DROP_SHADOW,
                    ..Default::default()
                });
                let src_x = if is_hovered { 267 } else { 256 };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: btn_rect,
                    source_rect: RectI::new(src_x, 256, 10, 10),
                    insets: BorderInsets::uniform(2.0),
                    scale: 4.0,
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            } else {
                let border_col = if is_hovered { theme_tokens.border_highlight } else { theme_tokens.border };
                let fill_col = if is_hovered { theme_tokens.secondary } else { theme_tokens.muted };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: btn_rect,
                    color: border_col,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(btn_rect.x + 2.0, btn_rect.y + 2.0, btn_rect.width - 4.0, btn_rect.height - 4.0),
                    color: fill_col,
                    ..Default::default()
                });
            }
        }

        // 4b. SDV_TAB: Top/Side Tab button
        if flags.contains(ComponentFlags::SDV_TAB) {
            let is_checked = flags.contains(ComponentFlags::IS_CHECKED);
            let is_hovered = flags.contains(ComponentFlags::IS_HOVERED);
            let y_shift = if is_checked { 4.0 } else { 0.0 };
            let tab_rect = RectF::new(rect.x, rect.y + y_shift, rect.width, rect.height);
            if theme_tokens.use_vanilla_9slice {
                let tint = if is_checked {
                    ColorRgba::WHITE
                } else if is_hovered {
                    ColorRgba::new(255, 240, 200, 255)
                } else {
                    ColorRgba::new(225, 205, 175, 255)
                };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: tab_rect,
                    source_rect: RectI::new(16, 368, 16, 16),
                    insets: BorderInsets::uniform(5.0),
                    scale: 4.0,
                    color: tint,
                    ..Default::default()
                });
            } else {
                let border_col = if is_checked || is_hovered { theme_tokens.border_highlight } else { theme_tokens.border };
                let fill_col = if is_checked { theme_tokens.card } else { theme_tokens.muted };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: tab_rect,
                    color: border_col,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(tab_rect.x + 2.0, tab_rect.y + 2.0, tab_rect.width - 4.0, tab_rect.height - 4.0),
                    color: fill_col,
                    ..Default::default()
                });
            }
        }

        // 4c. SHADCN_BADGE: Pill/Tag Badge (primary, secondary, outline, destructive, success)
        if flags.contains(ComponentFlags::SHADCN_BADGE) {
            let (bg, border) = match node_snapshot.visual.variant {
                1 => (theme_tokens.secondary, theme_tokens.border),
                2 => (ColorRgba::TRANSPARENT, theme_tokens.border_highlight),
                3 => (theme_tokens.destructive, theme_tokens.destructive),
                4 => (theme_tokens.success, theme_tokens.success),
                _ => (theme_tokens.primary, theme_tokens.border_highlight),
            };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: rect,
                color: border,
                ..Default::default()
            });
            if bg.a > 0 {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: bg,
                    ..Default::default()
                });
            }
        }

        // 4d. SHADCN_SWITCH: Modern Toggle Switch (52x28 track + 20x20 thumb)
        if flags.contains(ComponentFlags::SHADCN_SWITCH) {
            let is_checked = flags.contains(ComponentFlags::IS_CHECKED);
            let track_fill = if is_checked { theme_tokens.success } else { theme_tokens.muted };
            let border_col = if flags.contains(ComponentFlags::IS_HOVERED) {
                theme_tokens.border_highlight
            } else {
                theme_tokens.border
            };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: rect,
                color: border_col,
                ..Default::default()
            });
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                color: track_fill,
                ..Default::default()
            });
            let thumb_size = (rect.height - 8.0).max(16.0);
            let thumb_x = if is_checked {
                rect.x + rect.width - thumb_size - 4.0
            } else {
                rect.x + 4.0
            };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawRect as u32,
                dest_rect: RectF::new(thumb_x.round(), rect.y + 4.0, thumb_size, thumb_size),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }

        // 4e. LUCIDE_ICON: 16x16 Crisp Vector-to-Pixel Icon Atlas
        if flags.contains(ComponentFlags::LUCIDE_ICON) {
            let icon_name = node_snapshot.visual.icon_name.as_deref().unwrap_or("sparkles");
            let src = resolve_lucide_icon_rect(icon_name);
            let icon_col = if !theme_tokens.use_vanilla_9slice && node_snapshot.visual.text_color == ColorRgba::SDV_TEXT_DARK {
                theme_tokens.text_heading
            } else {
                node_snapshot.visual.text_color
            };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::LucideAtlas as u32,
                dest_rect: rect,
                source_rect: src,
                color: icon_col,
                ..Default::default()
            });
        }

        // 5. SDV_SLOT: 64x64 Inventory Item Slot + Item Icon + Stack Count
        if flags.contains(ComponentFlags::SDV_SLOT) {
            let is_hovered = flags.contains(ComponentFlags::IS_HOVERED);
            if theme_tokens.use_vanilla_9slice {
                let tint = if is_hovered {
                    ColorRgba::new(255, 245, 210, 255)
                } else {
                    ColorRgba::WHITE
                };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::MenuTiles as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(0, 320, 60, 60),
                    insets: BorderInsets::new(4.0, 10.0, 12.0, 4.0),
                    scale: 1.0,
                    color: tint,
                    ..Default::default()
                });
            } else {
                let border_col = if is_hovered { theme_tokens.border_highlight } else { theme_tokens.border };
                let fill_col = if is_hovered { theme_tokens.secondary } else { theme_tokens.muted };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: border_col,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: fill_col,
                    ..Default::default()
                });
            }

            // Draw item sprite from Maps/springobjects (24 items per row, 16x16 each)
            let item_idx = node_snapshot.visual.item_index;
            if item_idx >= 0 {
                let src_x = (item_idx % 24) * 16;
                let src_y = (item_idx / 24) * 16;
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawSprite as u32,
                    texture_id: TextureId::Objects as u32,
                    dest_rect: RectF::new(rect.x + 8.0, rect.y + 8.0, 48.0, 48.0),
                    source_rect: RectI::new(src_x, src_y, 16, 16),
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });

                // Quality star (Cursors [338, 400, 8, 8] silver, [346, 400, 8, 8] gold, [346, 392, 8, 8] iridium)
                let q = node_snapshot.visual.item_quality;
                if q > 0 {
                    let star_src = match q {
                        1 => RectI::new(338, 400, 8, 8),
                        2 => RectI::new(346, 400, 8, 8),
                        _ => RectI::new(346, 392, 8, 8),
                    };
                    self.render_commands.push(RenderCommand {
                        kind: RenderCommandKind::DrawSprite as u32,
                        texture_id: TextureId::Cursors as u32,
                        dest_rect: RectF::new(rect.x + 8.0, rect.y + rect.height - 24.0, 16.0, 16.0),
                        source_rect: star_src,
                        color: ColorRgba::WHITE,
                        ..Default::default()
                    });
                }

                // Stack count
                let count = node_snapshot.visual.item_count;
                if count > 1 {
                    let count_str = count.to_string();
                    let (off, len) = self.push_render_text(&count_str);
                    self.render_commands.push(RenderCommand {
                        kind: RenderCommandKind::DrawText as u32,
                        font_id: VanillaFont::Small as u32,
                        dest_rect: RectF::new(rect.x + rect.width - 28.0, rect.y + rect.height - 28.0, 24.0, 24.0),
                        color: ColorRgba::WHITE,
                        shadow_color: ColorRgba::SDV_TEXT_DARK,
                        shadow_offset_x: 2.0,
                        shadow_offset_y: 2.0,
                        scale: 0.75,
                        text_offset: off,
                        text_len: len,
                        ..Default::default()
                    });
                }
            }
        }

        // 6. SDV_CLOSE: Red 'X' close button (Cursors [337, 494, 12, 12])
        if flags.contains(ComponentFlags::SDV_CLOSE) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(
                    rect.x - grow * 0.5,
                    rect.y - grow * 0.5,
                    rect.width + grow,
                    rect.height + grow,
                ),
                source_rect: RectI::new(337, 494, 12, 12),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }

        // 7. SDV_CHECKBOX: Vanilla checkbox (Cursors [227, 425, 9, 9] / [236, 425, 9, 9])
        if flags.contains(ComponentFlags::SDV_CHECKBOX) {
            let src_x = if flags.contains(ComponentFlags::IS_CHECKED) { 236 } else { 227 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: rect,
                source_rect: RectI::new(src_x, 425, 9, 9),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }

        // 8. SDV_BAR: Progress / Energy bar with smooth color interpolation
        if flags.contains(ComponentFlags::SDV_BAR) {
            let ratio = (node_snapshot.visual.progress_value / node_snapshot.visual.progress_max).clamp(0.0, 1.0);
            if theme_tokens.use_vanilla_9slice {
                // Outer frame
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(403, 383, 6, 6),
                    insets: BorderInsets::uniform(2.0),
                    scale: 4.0,
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            } else {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: theme_tokens.border,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: theme_tokens.muted,
                    ..Default::default()
                });
            }
            // Inner fill (red -> gold -> green based on ratio, or theme success/primary)
            let fill_w = ((rect.width - 8.0) * ratio).max(0.0);
            if fill_w > 0.0 {
                let fill_color = if !theme_tokens.use_vanilla_9slice {
                    if ratio < 0.3 {
                        theme_tokens.destructive
                    } else if ratio < 0.65 {
                        theme_tokens.accent
                    } else {
                        theme_tokens.success
                    }
                } else if ratio < 0.3 {
                    ColorRgba::SDV_RED
                } else if ratio < 0.65 {
                    ColorRgba::SDV_GOLD
                } else {
                    ColorRgba::SDV_GREEN
                };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 4.0, rect.y + 4.0, fill_w, (rect.height - 8.0).max(4.0)),
                    color: fill_color,
                    ..Default::default()
                });
            }
        }

        // 8b. SDV_SLIDER: Interactive OptionsSlider (Track [403, 383, 6, 6] + Fill + Thumb [420, 441, 10, 6])
        if flags.contains(ComponentFlags::SDV_SLIDER) {
            let ratio = (node_snapshot.visual.progress_value / node_snapshot.visual.progress_max).clamp(0.0, 1.0);
            if theme_tokens.use_vanilla_9slice {
                // Track
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(403, 383, 6, 6),
                    insets: BorderInsets::uniform(2.0),
                    scale: 4.0,
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            } else {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: theme_tokens.border,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: theme_tokens.muted,
                    ..Default::default()
                });
            }
            // Active track fill
            let fill_w = ((rect.width - 8.0) * ratio).max(0.0);
            if fill_w > 0.0 {
                let fill_col = if theme_tokens.use_vanilla_9slice {
                    ColorRgba::SDV_GOLD
                } else {
                    theme_tokens.primary
                };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 4.0, rect.y + 4.0, fill_w, (rect.height - 8.0).max(4.0)),
                    color: fill_col,
                    ..Default::default()
                });
            }
            // Slider Thumb (40x24)
            let thumb_w = 40.0;
            let thumb_x = (rect.x + (rect.width - thumb_w) * ratio).round();
            if theme_tokens.use_vanilla_9slice {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawSprite as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: RectF::new(thumb_x, rect.y, thumb_w, rect.height),
                    source_rect: RectI::new(420, 441, 10, 6),
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            } else {
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(thumb_x, rect.y, thumb_w, rect.height),
                    color: theme_tokens.border_highlight,
                    ..Default::default()
                });
            }
        }

        // 8c. SDV_INPUT: Interactive Text Input Box (Cursors [403, 373, 9, 9] @ 4x OR Theme Input)
        if flags.contains(ComponentFlags::SDV_INPUT) {
            let is_focused = flags.contains(ComponentFlags::IS_FOCUSED);
            if theme_tokens.use_vanilla_9slice {
                let tint = if is_focused {
                    ColorRgba::new(255, 250, 215, 255)
                } else {
                    ColorRgba::WHITE
                };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: rect,
                    source_rect: RectI::new(403, 373, 9, 9),
                    insets: BorderInsets::uniform(2.0),
                    scale: 4.0,
                    color: tint,
                    ..Default::default()
                });
            } else {
                let border_col = if is_focused { theme_tokens.border_highlight } else { theme_tokens.border };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: rect,
                    color: border_col,
                    ..Default::default()
                });
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 2.0, rect.y + 2.0, rect.width - 4.0, rect.height - 4.0),
                    color: theme_tokens.muted,
                    ..Default::default()
                });
            }
            if is_focused {
                // Focus indicator bar on left edge
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawRect as u32,
                    dest_rect: RectF::new(rect.x + 6.0, rect.y + 8.0, 4.0, (rect.height - 16.0).max(8.0)),
                    color: ColorRgba::SDV_GOLD,
                    ..Default::default()
                });
            }
        }

        // 8d. SDV_ARROW_LEFT / RIGHT / UP / DOWN & SDV_TRASH & SDV_COIN
        if flags.contains(ComponentFlags::SDV_ARROW_LEFT) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(rect.x - grow * 0.5, rect.y - grow * 0.5, rect.width + grow, rect.height + grow),
                source_rect: RectI::new(352, 495, 12, 11),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }
        if flags.contains(ComponentFlags::SDV_ARROW_RIGHT) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(rect.x - grow * 0.5, rect.y - grow * 0.5, rect.width + grow, rect.height + grow),
                source_rect: RectI::new(365, 495, 12, 11),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }
        if flags.contains(ComponentFlags::SDV_ARROW_UP) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(rect.x - grow * 0.5, rect.y - grow * 0.5, rect.width + grow, rect.height + grow),
                source_rect: RectI::new(421, 459, 11, 12),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }
        if flags.contains(ComponentFlags::SDV_ARROW_DOWN) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(rect.x - grow * 0.5, rect.y - grow * 0.5, rect.width + grow, rect.height + grow),
                source_rect: RectI::new(421, 472, 11, 12),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }
        if flags.contains(ComponentFlags::SDV_TRASH) {
            let grow = if flags.contains(ComponentFlags::IS_HOVERED) { 4.0 } else { 0.0 };
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: RectF::new(rect.x - grow * 0.5, rect.y - grow * 0.5, rect.width + grow, rect.height + grow),
                source_rect: RectI::new(323, 433, 9, 10),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }
        if flags.contains(ComponentFlags::SDV_COIN) {
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::Cursors as u32,
                dest_rect: rect,
                source_rect: RectI::new(280, 412, 15, 14),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }

        // 8e. SDV_HEARTS: Friendship Heart Meter (Cursors [211, 428, 7, 6] filled / [218, 428, 7, 6] empty @ 4x)
        if flags.contains(ComponentFlags::SDV_HEARTS) {
            let max_h = node_snapshot.visual.hearts_max.clamp(1, 14);
            let filled_h = node_snapshot.visual.hearts_filled.clamp(0, max_h);
            let step = if max_h > 1 {
                ((rect.width - 28.0) / ((max_h - 1) as f32)).clamp(16.0, 28.0)
            } else {
                28.0
            };
            for i in 0..max_h {
                let hx = (rect.x + (i as f32) * step).round();
                let src_x = if i < filled_h { 211 } else { 218 };
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawSprite as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: RectF::new(hx, rect.y + 2.0, 28.0, 24.0),
                    source_rect: RectI::new(src_x, 428, 7, 6),
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            }
        }

        // 9. SDV_DIVIDER: Wooden horizontal separator
        if flags.contains(ComponentFlags::SDV_DIVIDER) {
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::DrawSprite as u32,
                texture_id: TextureId::MenuTiles as u32,
                dest_rect: rect,
                source_rect: RectI::new(64, 412, 64, 8),
                color: ColorRgba::WHITE,
                ..Default::default()
            });
        }

        // 10. Leaf Text Rendering (with wrapped lines and optional text shadow)
        if let Some(ref raw_text) = node_snapshot.text_content {
            let parent_is_focused_input = node_snapshot
                .parent
                .and_then(|p| self.arena.get(p))
                .map_or(false, |pn| {
                    pn.flags.contains(ComponentFlags::SDV_INPUT)
                        && pn.flags.contains(ComponentFlags::IS_FOCUSED)
                });
            let text_cow = if parent_is_focused_input {
                format!("{raw_text}|")
            } else {
                raw_text.clone()
            };

            let font_kind = node_snapshot.visual.font;
            let scale = node_snapshot.visual.font_scale;
            let raw_color = node_snapshot.visual.text_color;
            let color = if !theme_tokens.use_vanilla_9slice {
                if raw_color == ColorRgba::SDV_TEXT_DARK {
                    theme_tokens.text_main
                } else if raw_color == ColorRgba::SDV_DIALOGUE_BROWN {
                    theme_tokens.text_heading
                } else if raw_color == ColorRgba::SDV_MUTED {
                    theme_tokens.text_muted
                } else {
                    raw_color
                }
            } else {
                raw_color
            };
            let has_shadow = flags.contains(ComponentFlags::TEXT_SHADOW);
            let (sh_color, sh_off) = if has_shadow {
                let off = match font_kind {
                    VanillaFont::Dialogue => 3.0,
                    VanillaFont::Small => 2.0,
                    VanillaFont::Tiny => 1.0,
                };
                (ColorRgba::SDV_TEXT_SHADOW, off)
            } else {
                (ColorRgba::TRANSPARENT, 0.0)
            };

            let mut lines_buf: Vec<(String, f32)> = Vec::new();
            self.fonts[font_kind as usize].for_each_wrapped_line(
                &text_cow,
                rect.width + 2.0,
                scale,
                |line_str, _idx, y_off| {
                    lines_buf.push((line_str.to_string(), y_off));
                },
            );

            for (line_str, y_off) in lines_buf {
                let (off, len) = self.push_render_text(&line_str);
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawText as u32,
                    font_id: font_kind as u32,
                    dest_rect: RectF::new(rect.x, (rect.y + y_off).round(), rect.width, rect.height),
                    color,
                    shadow_color: sh_color,
                    shadow_offset_x: sh_off,
                    shadow_offset_y: sh_off,
                    scale,
                    text_offset: off,
                    text_len: len,
                    ..Default::default()
                });
            }
        }

        // 11. Scissor Clipping & ScrollView Offset around children
        let is_scroll = flags.contains(ComponentFlags::SDV_SCROLL);
        let clipped = flags.contains(ComponentFlags::CLIP_CHILDREN) || is_scroll;
        if clipped {
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::PushClipRect as u32,
                dest_rect: rect,
                ..Default::default()
            });
        }

        let mut content_bottom = 0.0f32;
        if is_scroll {
            for &child in &node_snapshot.children {
                if let Some(cn) = self.arena.get(child) {
                    if let Ok(cl) = self.taffy.layout(cn.taffy_id) {
                        content_bottom = content_bottom.max(cl.location.y + cl.size.height + 16.0);
                    }
                }
            }
            let max_s = (content_bottom - rect.height).max(0.0);
            if let Some(n) = self.arena.get_mut(key) {
                n.visual.max_scroll_y = max_s;
                n.visual.scroll_y = n.visual.scroll_y.clamp(0.0, max_s);
            }
        }

        let scroll_offset_y = if is_scroll {
            self.arena.get(key).map_or(0.0, |n| n.visual.scroll_y)
        } else {
            0.0
        };

        for child in node_snapshot.children {
            self.snap_and_emit_recursive(child, abs_x, abs_y - scroll_offset_y);
        }

        if clipped {
            self.render_commands.push(RenderCommand {
                kind: RenderCommandKind::PopClipRect as u32,
                ..Default::default()
            });
        }

        // Draw scrollbar on right edge of SDV_SCROLL when content overflows
        if is_scroll {
            let (scroll_y, max_scroll_y) = self
                .arena
                .get(key)
                .map_or((0.0, 0.0), |n| (n.visual.scroll_y, n.visual.max_scroll_y));
            if max_scroll_y > 1.0 {
                let bar_w = 20.0;
                let bar_x = rect.x + rect.width - bar_w - 4.0;
                let bar_y = rect.y + 4.0;
                let bar_h = (rect.height - 8.0).max(32.0);
                // Track
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawNineSlice as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: RectF::new(bar_x, bar_y, bar_w, bar_h),
                    source_rect: RectI::new(403, 383, 6, 6),
                    insets: BorderInsets::uniform(2.0),
                    scale: 3.0,
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
                // Thumb (Cursors [435, 463, 6, 10])
                let thumb_h = 36.0;
                let ratio = (scroll_y / max_scroll_y).clamp(0.0, 1.0);
                let thumb_y = (bar_y + (bar_h - thumb_h) * ratio).round();
                self.render_commands.push(RenderCommand {
                    kind: RenderCommandKind::DrawSprite as u32,
                    texture_id: TextureId::Cursors as u32,
                    dest_rect: RectF::new(bar_x, thumb_y, bar_w, thumb_h),
                    source_rect: RectI::new(435, 463, 6, 10),
                    color: ColorRgba::WHITE,
                    ..Default::default()
                });
            }
        }
    }

    fn push_render_text(&mut self, s: &str) -> (u32, u32) {
        let offset = self.render_text_pool.len() as u32;
        let bytes = s.as_bytes();
        self.render_text_pool.extend_from_slice(bytes);
        (offset, bytes.len() as u32)
    }

    pub fn current_render_view(&self) -> RenderSliceView {
        RenderSliceView {
            commands_ptr: self.render_commands.as_ptr(),
            commands_len: self.render_commands.len(),
            text_pool_ptr: self.render_text_pool.as_ptr(),
            text_pool_len: self.render_text_pool.len(),
        }
    }

    pub fn current_event_view(&self) -> EventSliceView {
        EventSliceView {
            events_ptr: self.events.as_ptr(),
            events_len: self.events.len(),
            text_pool_ptr: self.event_text_pool.as_ptr(),
            text_pool_len: self.event_text_pool.len(),
        }
    }
}

impl ElementNode {
    pub fn is_interactive(&self) -> bool {
        self.action_name.is_some()
            || self.flags.intersects(
                ComponentFlags::SDV_BUTTON
                    | ComponentFlags::SDV_CLOSE
                    | ComponentFlags::SDV_CHECKBOX
                    | ComponentFlags::SDV_SLOT
                    | ComponentFlags::SDV_SLIDER
                    | ComponentFlags::SDV_INPUT
                    | ComponentFlags::SDV_TAB
                    | ComponentFlags::SDV_ARROW_LEFT
                    | ComponentFlags::SDV_ARROW_RIGHT
                    | ComponentFlags::SDV_ARROW_UP
                    | ComponentFlags::SDV_ARROW_DOWN
                    | ComponentFlags::SDV_TRASH
                    | ComponentFlags::SHADCN_SWITCH,
            )
    }
}
