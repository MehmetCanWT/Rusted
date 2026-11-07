use bitflags::bitflags;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderCommandKind {
    DrawNineSlice = 0,
    DrawSprite = 1,
    DrawText = 2,
    DrawRect = 3,
    PushClipRect = 4,
    PopClipRect = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureId {
    None = 0,
    Cursors = 1,
    Cursors2 = 2,
    MenuTiles = 3,
    MenuTilesUncolored = 4,
    TextBox = 5,
    Objects = 6,
    LucideAtlas = 7,
    Custom = 255,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiThemeId {
    #[default]
    PelicanClassic = 0,
    ShadcnZincDark = 1,
    WizardArcane = 2,
    JojaCorp = 3,
    JunimoForest = 4,
    CatppuccinMocha = 5,
    TokyoNight = 6,
    Dracula = 7,
    Nord = 8,
    Cyberpunk = 9,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VanillaFont {
    #[default]
    Small = 0,
    Dialogue = 1,
    Tiny = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEventKind {
    Click = 0,
    HoverEnter = 1,
    HoverLeave = 2,
    ValueChanged = 3,
    CloseRequested = 4,
    SoundCue = 5,
    FocusInput = 6,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCueId {
    None = 0,
    Shiny4 = 1,      // Hover / focus snap
    BigSelect = 2,   // Primary button / tab click
    SmallSelect = 3, // Checkbox / minor toggle / slider step
    Dwop = 4,        // Slot click / badge pop
    Coin = 5,        // Reward / gold action
    Drumkit6 = 6,    // Menu close
    Cancel = 7,      // Disabled click
    Trashcan = 8,    // Trash button click
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectF {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    #[inline(always)]
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RectI {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl RectI {
    #[inline(always)]
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    pub const SDV_TEXT_DARK: Self = Self::new(34, 17, 34, 255);       // #221122
    pub const SDV_DIALOGUE_BROWN: Self = Self::new(86, 22, 12, 255);  // #56160C
    pub const SDV_TEXT_SHADOW: Self = Self::new(206, 156, 95, 255);   // #CE9C5F
    pub const SDV_DROP_SHADOW: Self = Self::new(0, 0, 0, 95);         // 37% black shadow
    pub const SDV_GOLD: Self = Self::new(255, 215, 0, 255);           // #FFD700
    pub const SDV_IRIDIUM: Self = Self::new(205, 95, 255, 255);       // #CD5FFF
    pub const SDV_GREEN: Self = Self::new(0, 185, 55, 255);           // Energy green
    pub const SDV_RED: Self = Self::new(220, 45, 35, 255);            // Health red
    pub const SDV_MUTED: Self = Self::new(115, 90, 75, 255);          // Muted brown-grey

    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BorderInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BorderInsets {
    #[inline(always)]
    pub const fn uniform(v: f32) -> Self {
        Self { top: v, right: v, bottom: v, left: v }
    }

    #[inline(always)]
    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
}

/// Blittable, fixed-layout RenderCommand shared zero-copy with C# via ReadOnlySpan<RenderCommand>.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderCommand {
    pub kind: u32,
    pub texture_id: u32,
    pub font_id: u32,
    pub scale: f32,
    pub dest_rect: RectF,
    pub source_rect: RectI,
    pub insets: BorderInsets,
    pub color: ColorRgba,
    pub shadow_color: ColorRgba,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub text_offset: u32,
    pub text_len: u32,
}

impl Default for RenderCommand {
    fn default() -> Self {
        Self {
            kind: RenderCommandKind::DrawRect as u32,
            texture_id: TextureId::None as u32,
            font_id: VanillaFont::Small as u32,
            scale: 1.0,
            dest_rect: RectF::default(),
            source_rect: RectI::default(),
            insets: BorderInsets::default(),
            color: ColorRgba::WHITE,
            shadow_color: ColorRgba::TRANSPARENT,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            text_offset: 0,
            text_len: 0,
        }
    }
}

/// Blittable Pull-Based UI Event read by C# in UpdateTicked.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UiEvent {
    pub node_key_bits: u64,
    pub kind: u32,
    pub sound_cue: u32,
    pub action_offset: u32,
    pub action_len: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub float_value: f32,
    pub int_value: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NativeGlyphMetric {
    pub character: u32,
    pub advance: f32,
    pub left_bearing: f32,
    pub right_bearing: f32,
}

#[repr(C)]
pub struct RenderSliceView {
    pub commands_ptr: *const RenderCommand,
    pub commands_len: usize,
    pub text_pool_ptr: *const u8,
    pub text_pool_len: usize,
}

#[repr(C)]
pub struct EventSliceView {
    pub events_ptr: *const UiEvent,
    pub events_len: usize,
    pub text_pool_ptr: *const u8,
    pub text_pool_len: usize,
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DirtyFlags: u8 {
        const RENDER_DIRTY       = 0b0000_0001;
        const LAYOUT_DIRTY       = 0b0000_0010;
        const HIERARCHY_DIRTY    = 0b0000_0100;
        const TEXT_MEASURE_DIRTY = 0b0000_1000;
    }
}

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ComponentFlags: u32 {
        const NONE            = 0;
        const SDV_BOX         = 1 << 0;  // Cursors [384, 373, 18, 18] @ 4x
        const SDV_PANEL       = 1 << 1;  // MenuTiles ControlBorder [0, 256, 60, 60]
        const SDV_PARCHMENT   = 1 << 2;  // Cursors BannerBackground [325, 318, 25, 18]
        const SDV_BUTTON      = 1 << 3;  // Cursors ButtonDark / ButtonLight
        const SDV_SLOT        = 1 << 4;  // MenuTiles MenuSlotInset [0, 320, 60, 60]
        const SDV_CLOSE       = 1 << 5;  // Cursors CloseButton [337, 494, 12, 12]
        const SDV_CHECKBOX    = 1 << 6;  // Cursors Checkbox [227/236, 425, 9, 9]
        const SDV_BAR         = 1 << 7;  // Progress / Energy bar
        const SDV_DIVIDER     = 1 << 8;  // MenuTiles ThinHDivider [64, 412, 64, 8]
        const SDV_SLIDER      = 1 << 9;  // Interactive Horizontal Slider
        const SDV_INPUT       = 1 << 10; // Focusable TextBox Input
        const SDV_TAB         = 1 << 11; // Tab Header Button
        const SDV_SCROLL      = 1 << 12; // Scrollable View with Scissor Clip & Scrollbar
        const SDV_ARROW_LEFT  = 1 << 13; // Cursors SmallLeftArrow [352, 495, 12, 11]
        const SDV_ARROW_RIGHT = 1 << 14; // Cursors SmallRightArrow [365, 495, 12, 11]
        const SDV_ARROW_UP    = 1 << 15; // Cursors SmallUpArrow [421, 459, 11, 12]
        const SDV_ARROW_DOWN  = 1 << 16; // Cursors SmallDownArrow [421, 472, 11, 12]
        const SDV_TRASH       = 1 << 17; // Cursors TinyTrashCan [323, 433, 9, 10]
        const SDV_HEARTS      = 1 << 18; // Cursors Friendship Hearts [211/218, 428, 7, 6]
        const SDV_COIN        = 1 << 19; // Cursors Gold Coin [280, 412, 15, 14]
        const IS_HOVERED      = 1 << 20;
        const IS_ACTIVE       = 1 << 21;
        const IS_FOCUSED      = 1 << 22;
        const IS_CHECKED      = 1 << 23;
        const CLIP_CHILDREN   = 1 << 24;
        const TEXT_SHADOW     = 1 << 25;
        const SHADCN_BADGE    = 1 << 26; // Shadcn/UI Pill Badge (<badge>)
        const SHADCN_SWITCH   = 1 << 27; // Shadcn/UI Toggle Switch (<switch>)
        const LUCIDE_ICON     = 1 << 28; // Lucide Vector/Pixel Icon (<icon>)
        const SHADCN_CARD     = 1 << 29; // Shadcn/UI Surface Card (<card>)
    }
}
