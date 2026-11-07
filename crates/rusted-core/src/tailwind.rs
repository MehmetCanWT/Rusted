use taffy::prelude::*;
use taffy::style::Overflow;
use crate::types::{ColorRgba, ComponentFlags, VanillaFont};

#[derive(Clone, Debug)]
pub struct VisualProps {
    pub bg_color: Option<ColorRgba>,
    pub text_color: ColorRgba,
    pub font: VanillaFont,
    pub font_scale: f32,
    pub progress_value: f32,
    pub progress_max: f32,
    pub item_index: i32,
    pub item_count: i32,
    pub item_quality: u8,
    pub hearts_filled: i32,
    pub hearts_max: i32,
    pub scroll_y: f32,
    pub max_scroll_y: f32,
    pub variant: u8,
    pub icon_name: Option<String>,
}

impl Default for VisualProps {
    fn default() -> Self {
        Self {
            bg_color: None,
            text_color: ColorRgba::SDV_TEXT_DARK,
            font: VanillaFont::Small,
            font_scale: 1.0,
            progress_value: 0.0,
            progress_max: 100.0,
            item_index: -1,
            item_count: 0,
            item_quality: 0,
            hearts_filled: 4,
            hearts_max: 5,
            scroll_y: 0.0,
            max_scroll_y: 0.0,
            variant: 0,
            icon_name: None,
        }
    }
}

/// Applies space-separated Tailwind CSS & Pelican Theme utility classes to a Taffy Style + ComponentFlags + VisualProps.
pub fn apply_tailwind_classes(
    classes: &str,
    style: &mut Style,
    flags: &mut ComponentFlags,
    visual: &mut VisualProps,
) {
    for token in classes.split_whitespace() {
        apply_single_class(token, style, flags, visual);
    }

    // Guarantee ScrollView always preserves Taffy scroll overflow and right padding for the scrollbar track
    if flags.contains(ComponentFlags::SDV_SCROLL) {
        style.overflow.y = Overflow::Scroll;
        style.padding.right = LengthPercentage::Length(36.0);
    }
}

pub fn apply_single_class(
    class_name: &str,
    style: &mut Style,
    flags: &mut ComponentFlags,
    visual: &mut VisualProps,
) {
    const STEP: f32 = 4.0;

    match class_name {
        // Display
        "flex" => style.display = Display::Flex,
        "grid" => style.display = Display::Grid,
        "block" => style.display = Display::Block,
        "hidden" => style.display = Display::None,

        // Flex Direction & Wrap
        "flex-row" => {
            style.display = Display::Flex;
            style.flex_direction = FlexDirection::Row;
        }
        "flex-col" => {
            style.display = Display::Flex;
            style.flex_direction = FlexDirection::Column;
        }
        "flex-wrap" => style.flex_wrap = FlexWrap::Wrap,
        "flex-nowrap" => style.flex_wrap = FlexWrap::NoWrap,

        // Flex Grow / Shrink
        "flex-1" => {
            style.flex_grow = 1.0;
            style.flex_shrink = 1.0;
            style.flex_basis = Dimension::Length(0.0);
            style.min_size.width = Dimension::Length(0.0);
        }
        "grow" => style.flex_grow = 1.0,
        "grow-0" => style.flex_grow = 0.0,
        "shrink" => style.flex_shrink = 1.0,
        "shrink-0" => style.flex_shrink = 0.0,
        "min-w-0" => style.min_size.width = Dimension::Length(0.0),
        "min-h-0" => style.min_size.height = Dimension::Length(0.0),

        // Align Items & Self
        "items-start" => style.align_items = Some(AlignItems::FlexStart),
        "items-center" => style.align_items = Some(AlignItems::Center),
        "items-end" => style.align_items = Some(AlignItems::FlexEnd),
        "items-stretch" => style.align_items = Some(AlignItems::Stretch),

        "self-start" => style.align_self = Some(AlignSelf::FlexStart),
        "self-center" => style.align_self = Some(AlignSelf::Center),
        "self-end" => style.align_self = Some(AlignSelf::FlexEnd),
        "self-stretch" => style.align_self = Some(AlignSelf::Stretch),

        // Justify Content
        "justify-start" => style.justify_content = Some(JustifyContent::FlexStart),
        "justify-center" => style.justify_content = Some(JustifyContent::Center),
        "justify-between" => style.justify_content = Some(JustifyContent::SpaceBetween),
        "justify-around" => style.justify_content = Some(JustifyContent::SpaceAround),
        "justify-evenly" => style.justify_content = Some(JustifyContent::SpaceEvenly),
        "justify-end" => style.justify_content = Some(JustifyContent::FlexEnd),

        // Positioning
        "relative" => style.position = Position::Relative,
        "absolute" | "fixed" => style.position = Position::Absolute,

        // Sizing keywords
        "w-full" => style.size.width = Dimension::Percent(1.0),
        "h-full" => style.size.height = Dimension::Percent(1.0),
        "w-auto" => style.size.width = Dimension::Auto,
        "h-auto" => style.size.height = Dimension::Auto,

        // Overflow / Scissor Clipping
        "overflow-hidden" => {
            flags.insert(ComponentFlags::CLIP_CHILDREN);
            style.overflow.x = Overflow::Hidden;
            style.overflow.y = Overflow::Hidden;
        }
        "overflow-scroll" | "sdv-scroll" => {
            flags.insert(ComponentFlags::CLIP_CHILDREN | ComponentFlags::SDV_SCROLL);
            style.display = Display::Flex;
            style.flex_direction = FlexDirection::Column;
            style.overflow.y = Overflow::Scroll;
            style.padding = Rect {
                left: LengthPercentage::Length(16.0),
                right: LengthPercentage::Length(36.0),
                top: LengthPercentage::Length(12.0),
                bottom: LengthPercentage::Length(12.0),
            };
        }

        // Typography & Stardew Fonts
        "font-small" => visual.font = VanillaFont::Small,
        "font-dialogue" => {
            visual.font = VanillaFont::Dialogue;
            flags.insert(ComponentFlags::TEXT_SHADOW);
        }
        "font-tiny" => visual.font = VanillaFont::Tiny,
        "text-shadow" => flags.insert(ComponentFlags::TEXT_SHADOW),

        // Stardew & Shadcn Semantic Text Colors
        "text-sdv-dark" | "text-foreground" => visual.text_color = ColorRgba::SDV_TEXT_DARK,
        "text-sdv-brown" | "text-primary" => visual.text_color = ColorRgba::SDV_DIALOGUE_BROWN,
        "text-sdv-gold" | "text-amber" => visual.text_color = ColorRgba::SDV_GOLD,
        "text-sdv-iridium" | "text-arcane" => visual.text_color = ColorRgba::SDV_IRIDIUM,
        "text-sdv-green" | "text-emerald" => visual.text_color = ColorRgba::SDV_GREEN,
        "text-sdv-red" | "text-destructive" => visual.text_color = ColorRgba::SDV_RED,
        "text-sdv-muted" | "text-muted-foreground" => visual.text_color = ColorRgba::SDV_MUTED,
        "text-white" | "text-zinc-50" => visual.text_color = ColorRgba::WHITE,
        "text-joja-cyan" => visual.text_color = ColorRgba::new(56, 189, 248, 255),
        "text-junimo-lime" => visual.text_color = ColorRgba::new(236, 252, 203, 255),

        // 5 Theme Surface / Tint Presets (Pelican, Zinc Dark, Wizard Arcane, Joja Corp, Junimo Forest)
        "theme-pelican" => visual.bg_color = None,
        "theme-zinc" | "bg-zinc-900" => visual.bg_color = Some(ColorRgba::new(160, 165, 180, 255)),
        "theme-arcane" | "bg-arcane" => visual.bg_color = Some(ColorRgba::new(205, 175, 255, 255)),
        "theme-joja" | "bg-joja" => visual.bg_color = Some(ColorRgba::new(165, 220, 255, 255)),
        "theme-junimo" | "bg-junimo" => visual.bg_color = Some(ColorRgba::new(185, 245, 195, 255)),

        // Pelican Native Theme Presets
        "sdv-box" => {
            flags.insert(ComponentFlags::SDV_BOX);
            let pad = LengthPercentage::Length(24.0);
            style.padding = Rect { left: pad, right: pad, top: pad, bottom: pad };
        }
        "sdv-panel" => {
            flags.insert(ComponentFlags::SDV_PANEL);
            let pad = LengthPercentage::Length(16.0);
            style.padding = Rect { left: pad, right: pad, top: pad, bottom: pad };
        }
        "sdv-parchment" | "sdv-banner" => {
            flags.insert(ComponentFlags::SDV_PARCHMENT);
            style.flex_shrink = 0.0;
            style.padding = Rect {
                left: LengthPercentage::Length(20.0),
                right: LengthPercentage::Length(20.0),
                top: LengthPercentage::Length(10.0),
                bottom: LengthPercentage::Length(10.0),
            };
        }
        "sdv-btn" => {
            flags.insert(ComponentFlags::SDV_BUTTON);
            style.display = Display::Flex;
            style.align_items = Some(AlignItems::Center);
            style.justify_content = Some(JustifyContent::Center);
            style.flex_shrink = 0.0;
            style.padding = Rect {
                left: LengthPercentage::Length(16.0),
                right: LengthPercentage::Length(16.0),
                top: LengthPercentage::Length(10.0),
                bottom: LengthPercentage::Length(10.0),
            };
        }
        "sdv-tab" => {
            flags.insert(ComponentFlags::SDV_TAB);
            style.display = Display::Flex;
            style.align_items = Some(AlignItems::Center);
            style.justify_content = Some(JustifyContent::Center);
            style.flex_shrink = 0.0;
            style.padding = Rect {
                left: LengthPercentage::Length(16.0),
                right: LengthPercentage::Length(16.0),
                top: LengthPercentage::Length(10.0),
                bottom: LengthPercentage::Length(10.0),
            };
        }
        "sdv-slot" => {
            flags.insert(ComponentFlags::SDV_SLOT);
            style.size = Size {
                width: Dimension::Length(64.0),
                height: Dimension::Length(64.0),
            };
            style.flex_shrink = 0.0;
        }
        "sdv-close" => {
            flags.insert(ComponentFlags::SDV_CLOSE);
            style.size = Size {
                width: Dimension::Length(48.0),
                height: Dimension::Length(48.0),
            };
            style.flex_shrink = 0.0;
        }
        "sdv-checkbox" => {
            flags.insert(ComponentFlags::SDV_CHECKBOX);
            style.size = Size {
                width: Dimension::Length(36.0),
                height: Dimension::Length(36.0),
            };
            style.flex_shrink = 0.0;
        }
        "sdv-slider" => {
            flags.insert(ComponentFlags::SDV_SLIDER);
            style.flex_shrink = 0.0;
            if style.size.height == Dimension::Auto {
                style.size.height = Dimension::Length(28.0);
            }
            if style.size.width == Dimension::Auto {
                style.size.width = Dimension::Percent(1.0);
            }
        }
        "sdv-input" => {
            flags.insert(ComponentFlags::SDV_INPUT);
            style.display = Display::Flex;
            style.align_items = Some(AlignItems::Center);
            style.min_size.width = Dimension::Length(0.0);
            style.padding = Rect {
                left: LengthPercentage::Length(14.0),
                right: LengthPercentage::Length(14.0),
                top: LengthPercentage::Length(8.0),
                bottom: LengthPercentage::Length(8.0),
            };
            if style.size.height == Dimension::Auto {
                style.size.height = Dimension::Length(44.0);
            }
        }
        "sdv-bar" => {
            flags.insert(ComponentFlags::SDV_BAR);
            style.flex_shrink = 0.0;
            if style.size.height == Dimension::Auto {
                style.size.height = Dimension::Length(24.0);
            }
        }
        "sdv-divider" => {
            flags.insert(ComponentFlags::SDV_DIVIDER);
            style.size.width = Dimension::Percent(1.0);
            style.size.height = Dimension::Length(12.0);
            style.flex_shrink = 0.0;
        }
        "sdv-arrow-left" => {
            flags.insert(ComponentFlags::SDV_ARROW_LEFT);
            style.size = Size { width: Dimension::Length(44.0), height: Dimension::Length(40.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-arrow-right" => {
            flags.insert(ComponentFlags::SDV_ARROW_RIGHT);
            style.size = Size { width: Dimension::Length(44.0), height: Dimension::Length(40.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-arrow-up" => {
            flags.insert(ComponentFlags::SDV_ARROW_UP);
            style.size = Size { width: Dimension::Length(40.0), height: Dimension::Length(44.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-arrow-down" => {
            flags.insert(ComponentFlags::SDV_ARROW_DOWN);
            style.size = Size { width: Dimension::Length(40.0), height: Dimension::Length(44.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-trash" => {
            flags.insert(ComponentFlags::SDV_TRASH);
            style.size = Size { width: Dimension::Length(36.0), height: Dimension::Length(40.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-hearts" => {
            flags.insert(ComponentFlags::SDV_HEARTS);
            style.size = Size { width: Dimension::Length(260.0), height: Dimension::Length(28.0) };
            style.flex_shrink = 1.0;
            style.min_size.width = Dimension::Length(140.0);
        }
        "sdv-coin" => {
            flags.insert(ComponentFlags::SDV_COIN);
            style.size = Size { width: Dimension::Length(32.0), height: Dimension::Length(32.0) };
            style.flex_shrink = 0.0;
        }
        "sdv-badge" | "badge" | "badge-default" => {
            flags.insert(ComponentFlags::SHADCN_BADGE);
            visual.variant = 0;
            style.display = Display::Flex;
            style.align_items = Some(AlignItems::Center);
            style.justify_content = Some(JustifyContent::Center);
            style.flex_shrink = 0.0;
            style.padding = Rect {
                left: LengthPercentage::Length(10.0),
                right: LengthPercentage::Length(10.0),
                top: LengthPercentage::Length(4.0),
                bottom: LengthPercentage::Length(4.0),
            };
        }
        "badge-secondary" => {
            flags.insert(ComponentFlags::SHADCN_BADGE);
            visual.variant = 1;
        }
        "badge-outline" => {
            flags.insert(ComponentFlags::SHADCN_BADGE);
            visual.variant = 2;
        }
        "badge-destructive" => {
            flags.insert(ComponentFlags::SHADCN_BADGE);
            visual.variant = 3;
        }
        "badge-success" => {
            flags.insert(ComponentFlags::SHADCN_BADGE);
            visual.variant = 4;
        }
        "sdv-switch" | "shadcn-switch" => {
            flags.insert(ComponentFlags::SHADCN_SWITCH);
            style.size = Size { width: Dimension::Length(52.0), height: Dimension::Length(28.0) };
            style.flex_shrink = 0.0;
        }
        "shadcn-card" | "card" => {
            flags.insert(ComponentFlags::SHADCN_CARD);
            let pad = LengthPercentage::Length(16.0);
            style.padding = Rect { left: pad, right: pad, top: pad, bottom: pad };
        }
        "lucide-icon" | "icon" => {
            flags.insert(ComponentFlags::LUCIDE_ICON);
            if style.size.width == Dimension::Auto {
                style.size = Size { width: Dimension::Length(24.0), height: Dimension::Length(24.0) };
            }
            style.flex_shrink = 0.0;
        }
        "lucide-sm" => {
            flags.insert(ComponentFlags::LUCIDE_ICON);
            style.size = Size { width: Dimension::Length(20.0), height: Dimension::Length(20.0) };
            style.flex_shrink = 0.0;
        }
        "lucide-md" => {
            flags.insert(ComponentFlags::LUCIDE_ICON);
            style.size = Size { width: Dimension::Length(24.0), height: Dimension::Length(24.0) };
            style.flex_shrink = 0.0;
        }
        "lucide-lg" => {
            flags.insert(ComponentFlags::LUCIDE_ICON);
            style.size = Size { width: Dimension::Length(32.0), height: Dimension::Length(32.0) };
            style.flex_shrink = 0.0;
        }

        _ => parse_parametric_class(class_name, style, flags, visual, STEP),
    }
}

fn parse_parametric_class(
    name: &str,
    style: &mut Style,
    flags: &mut ComponentFlags,
    visual: &mut VisualProps,
    step: f32,
) {
    if let Some(icon) = name.strip_prefix("lucide-") {
        flags.insert(ComponentFlags::LUCIDE_ICON);
        visual.icon_name = Some(icon.to_string());
        if style.size.width == Dimension::Auto {
            style.size = Size { width: Dimension::Length(24.0), height: Dimension::Length(24.0) };
        }
        style.flex_shrink = 0.0;
        return;
    }
    if let Some(v) = name.strip_prefix("gap-") {
        if let Ok(n) = v.parse::<f32>() {
            let len = LengthPercentage::Length(n * step);
            style.gap = Size { width: len, height: len };
        }
    } else if let Some(v) = name.strip_prefix("p-") {
        if let Ok(n) = v.parse::<f32>() {
            let len = LengthPercentage::Length(n * step);
            style.padding = Rect { left: len, right: len, top: len, bottom: len };
        }
    } else if let Some(v) = name.strip_prefix("px-") {
        if let Ok(n) = v.parse::<f32>() {
            let len = LengthPercentage::Length(n * step);
            style.padding.left = len;
            style.padding.right = len;
        }
    } else if let Some(v) = name.strip_prefix("py-") {
        if let Ok(n) = v.parse::<f32>() {
            let len = LengthPercentage::Length(n * step);
            style.padding.top = len;
            style.padding.bottom = len;
        }
    } else if let Some(v) = name.strip_prefix("pt-") {
        if let Ok(n) = v.parse::<f32>() {
            style.padding.top = LengthPercentage::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("pb-") {
        if let Ok(n) = v.parse::<f32>() {
            style.padding.bottom = LengthPercentage::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("pl-") {
        if let Ok(n) = v.parse::<f32>() {
            style.padding.left = LengthPercentage::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("pr-") {
        if let Ok(n) = v.parse::<f32>() {
            style.padding.right = LengthPercentage::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("m-") {
        if let Ok(n) = v.parse::<f32>() {
            let len = LengthPercentageAuto::Length(n * step);
            style.margin = Rect { left: len, right: len, top: len, bottom: len };
        }
    } else if let Some(v) = name.strip_prefix("mt-") {
        if let Ok(n) = v.parse::<f32>() {
            style.margin.top = LengthPercentageAuto::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("mb-") {
        if let Ok(n) = v.parse::<f32>() {
            style.margin.bottom = LengthPercentageAuto::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("w-") {
        if let Some(pct) = parse_fraction_percent(v) {
            style.size.width = Dimension::Percent(pct);
        } else if let Some(px) = v.strip_suffix("px").and_then(|s| s.parse::<f32>().ok()) {
            style.size.width = Dimension::Length(px);
        } else if let Ok(n) = v.parse::<f32>() {
            let px = if n >= 100.0 { n } else { n * step };
            style.size.width = Dimension::Length(px);
        }
    } else if let Some(v) = name.strip_prefix("h-") {
        if let Some(pct) = parse_fraction_percent(v) {
            style.size.height = Dimension::Percent(pct);
        } else if let Some(px) = v.strip_suffix("px").and_then(|s| s.parse::<f32>().ok()) {
            style.size.height = Dimension::Length(px);
        } else if let Ok(n) = v.parse::<f32>() {
            let px = if n >= 100.0 { n } else { n * step };
            style.size.height = Dimension::Length(px);
        }
    } else if let Some(v) = name.strip_prefix("top-") {
        if let Ok(n) = v.parse::<f32>() {
            style.inset.top = LengthPercentageAuto::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("right-") {
        if let Ok(n) = v.parse::<f32>() {
            style.inset.right = LengthPercentageAuto::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("bottom-") {
        if let Ok(n) = v.parse::<f32>() {
            style.inset.bottom = LengthPercentageAuto::Length(n * step);
        }
    } else if let Some(v) = name.strip_prefix("left-") {
        if let Ok(n) = v.parse::<f32>() {
            style.inset.left = LengthPercentageAuto::Length(n * step);
        }
    }
}

fn parse_fraction_percent(s: &str) -> Option<f32> {
    let (num, den) = s.split_once('/')?;
    let n = num.parse::<f32>().ok()?;
    let d = den.parse::<f32>().ok()?;
    if d > 0.0 {
        Some(n / d)
    } else {
        None
    }
}
