use crate::types::RectI;

/// Maps Lucide icon names to 16x16 grid cells inside `TextureId::LucideAtlas` (16 icons per row, 16x16 each).
pub fn resolve_lucide_icon_rect(name: &str) -> RectI {
    let idx: i32 = match name {
        "sparkles" | "magic" => 0,
        "cpu" | "chip" | "rust" => 1,
        "palette" | "theme" => 2,
        "code" | "tsx" | "react" => 3,
        "zap" | "bolt" | "fast" => 4,
        "layers" | "stack" => 5,
        "sliders" | "controls" => 6,
        "package" | "box" | "inventory" => 7,
        "heart" => 8,
        "star" => 9,
        "shield" => 10,
        "sword" => 11,
        "flame" | "fire" => 12,
        "terminal" | "console" => 13,
        "check" | "verified" => 14,
        "settings" | "cog" => 15,
        _ => 0,
    };
    let col = idx % 16;
    let row = idx / 16;
    RectI::new(col * 16, row * 16, 16, 16)
}
