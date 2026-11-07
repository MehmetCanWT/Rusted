use crate::types::{ColorRgba, UiThemeId};

#[derive(Clone, Copy, Debug)]
pub struct ThemeTokens {
    pub id: UiThemeId,
    pub name: &'static str,
    pub use_vanilla_9slice: bool,
    pub box_tint: ColorRgba,
    pub panel_tint: ColorRgba,
    pub background: ColorRgba,
    pub card: ColorRgba,
    pub muted: ColorRgba,
    pub border: ColorRgba,
    pub border_highlight: ColorRgba,
    pub primary: ColorRgba,
    pub primary_fg: ColorRgba,
    pub secondary: ColorRgba,
    pub accent: ColorRgba,
    pub destructive: ColorRgba,
    pub success: ColorRgba,
    pub text_main: ColorRgba,
    pub text_heading: ColorRgba,
    pub text_muted: ColorRgba,
}

impl ThemeTokens {
    pub const fn for_theme(theme: UiThemeId) -> Self {
        match theme {
            UiThemeId::PelicanClassic => Self {
                id: UiThemeId::PelicanClassic,
                name: "Pelican Classic",
                use_vanilla_9slice: true,
                box_tint: ColorRgba::WHITE,
                panel_tint: ColorRgba::WHITE,
                background: ColorRgba::new(245, 213, 151, 255),     // #F5D597 Warm Parchment
                card: ColorRgba::new(255, 235, 198, 255),           // #FFEBC6 Soft Cream
                muted: ColorRgba::new(225, 182, 118, 255),          // #E1B676 Recessed Tan
                border: ColorRgba::new(86, 22, 12, 255),            // #56160C Deep Oak
                border_highlight: ColorRgba::new(220, 123, 5, 255), // #DC7B05 Amber Bevel
                primary: ColorRgba::new(255, 165, 0, 255),          // #FFA500 Gold
                primary_fg: ColorRgba::new(34, 17, 34, 255),        // #221122
                secondary: ColorRgba::new(186, 134, 78, 255),       // #BA864E
                accent: ColorRgba::new(255, 215, 0, 255),           // #FFD700
                destructive: ColorRgba::new(220, 45, 35, 255),      // #DC2D23
                success: ColorRgba::new(34, 177, 76, 255),          // #22B14C
                text_main: ColorRgba::new(34, 17, 34, 255),         // #221122
                text_heading: ColorRgba::new(86, 22, 12, 255),      // #56160C
                text_muted: ColorRgba::new(115, 90, 75, 255),       // #735A4B
            },
            UiThemeId::ShadcnZincDark => Self {
                id: UiThemeId::ShadcnZincDark,
                name: "Shadcn Zinc Dark",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(24, 24, 27, 255),
                panel_tint: ColorRgba::new(39, 39, 42, 255),
                background: ColorRgba::new(9, 9, 11, 248),          // #09090B Zinc-950
                card: ColorRgba::new(24, 24, 27, 255),              // #18181B Zinc-900
                muted: ColorRgba::new(39, 39, 42, 255),             // #27272A Zinc-800
                border: ColorRgba::new(63, 63, 70, 255),            // #3F3F46 Zinc-700
                border_highlight: ColorRgba::new(82, 82, 91, 255),  // #52525B Zinc-600
                primary: ColorRgba::new(16, 185, 129, 255),         // #10B981 Emerald-500
                primary_fg: ColorRgba::new(9, 9, 11, 255),
                secondary: ColorRgba::new(63, 63, 70, 255),
                accent: ColorRgba::new(99, 102, 241, 255),          // #6366F1 Indigo-500
                destructive: ColorRgba::new(239, 68, 68, 255),      // #EF4444 Red-500
                success: ColorRgba::new(16, 185, 129, 255),         // #10B981 Emerald-500
                text_main: ColorRgba::new(244, 244, 245, 255),      // #F4F4F5 Zinc-100
                text_heading: ColorRgba::new(16, 185, 129, 255),    // #10B981 Emerald-500
                text_muted: ColorRgba::new(161, 161, 170, 255),     // #A1A1AA Zinc-400
            },
            UiThemeId::WizardArcane => Self {
                id: UiThemeId::WizardArcane,
                name: "Wizard Arcane",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(26, 16, 37, 255),
                panel_tint: ColorRgba::new(45, 27, 66, 255),
                background: ColorRgba::new(15, 8, 29, 248),         // #0F081D Astral Void
                card: ColorRgba::new(26, 16, 37, 255),              // #1A1025 Rune Table
                muted: ColorRgba::new(45, 27, 66, 255),             // #2D1B42 Mystic Plum
                border: ColorRgba::new(76, 40, 130, 255),           // #4C2882 Amethyst
                border_highlight: ColorRgba::new(168, 85, 247, 255),// #A855F7 Iridium Glow
                primary: ColorRgba::new(255, 215, 0, 255),          // #FFD700 Starlight Gold
                primary_fg: ColorRgba::new(26, 16, 37, 255),
                secondary: ColorRgba::new(107, 33, 168, 255),       // #6B21A8 Purple-800
                accent: ColorRgba::new(192, 132, 252, 255),         // #C084FC Purple-400
                destructive: ColorRgba::new(244, 63, 94, 255),      // #F43F5E Rose-500
                success: ColorRgba::new(52, 211, 153, 255),         // #34D399 Emerald-400
                text_main: ColorRgba::new(243, 232, 255, 255),      // #F3E8FF Lilac White
                text_heading: ColorRgba::new(255, 215, 0, 255),     // #FFD700 Starlight Gold
                text_muted: ColorRgba::new(192, 132, 252, 255),     // #C084FC
            },
            UiThemeId::JojaCorp => Self {
                id: UiThemeId::JojaCorp,
                name: "Joja Corp",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(15, 23, 42, 255),
                panel_tint: ColorRgba::new(30, 41, 59, 255),
                background: ColorRgba::new(15, 23, 42, 248),        // #0F172A Slate-900
                card: ColorRgba::new(30, 41, 59, 255),              // #1E293B Slate-800
                muted: ColorRgba::new(51, 65, 85, 255),             // #334155 Slate-700
                border: ColorRgba::new(2, 132, 199, 255),           // #0284C7 Joja Blue
                border_highlight: ColorRgba::new(56, 189, 248, 255),// #38BDF8 Sky-400
                primary: ColorRgba::new(2, 132, 199, 255),          // #0284C7 Joja Blue
                primary_fg: ColorRgba::new(248, 250, 252, 255),
                secondary: ColorRgba::new(51, 65, 85, 255),
                accent: ColorRgba::new(56, 189, 248, 255),          // #38BDF8 Cyan
                destructive: ColorRgba::new(225, 29, 72, 255),      // #E11D48 Rose-600
                success: ColorRgba::new(34, 197, 94, 255),          // #22C55E
                text_main: ColorRgba::new(241, 245, 249, 255),      // #F1F5F9 Slate-100
                text_heading: ColorRgba::new(56, 189, 248, 255),    // #38BDF8 Joja Cyan
                text_muted: ColorRgba::new(148, 163, 184, 255),     // #94A3B8 Slate-400
            },
            UiThemeId::JunimoForest => Self {
                id: UiThemeId::JunimoForest,
                name: "Junimo Forest",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(20, 38, 28, 255),
                panel_tint: ColorRgba::new(30, 56, 41, 255),
                background: ColorRgba::new(20, 38, 28, 248),        // #14261C Deep Moss
                card: ColorRgba::new(30, 56, 41, 255),              // #1E3829 Sage Canopy
                muted: ColorRgba::new(42, 77, 56, 255),             // #2A4D38 Forest Floor
                border: ColorRgba::new(74, 124, 89, 255),           // #4A7C59 Vine Green
                border_highlight: ColorRgba::new(134, 239, 172, 255),// #86EFAC Leaf Glow
                primary: ColorRgba::new(244, 63, 94, 255),          // #F43F5E Junimo Berry
                primary_fg: ColorRgba::new(255, 255, 255, 255),
                secondary: ColorRgba::new(52, 101, 72, 255),
                accent: ColorRgba::new(253, 224, 71, 255),          // #FDE047 Star Yellow
                destructive: ColorRgba::new(239, 68, 68, 255),
                success: ColorRgba::new(74, 222, 128, 255),         // #4ADE80
                text_main: ColorRgba::new(236, 252, 203, 255),      // #ECFCCB Dewdrop Cream
                text_heading: ColorRgba::new(253, 224, 71, 255),    // #FDE047 Junimo Yellow
                text_muted: ColorRgba::new(163, 230, 53, 255),      // #A3E635 Lime-400
            },
            UiThemeId::CatppuccinMocha => Self {
                id: UiThemeId::CatppuccinMocha,
                name: "Catppuccin Mocha",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(30, 30, 46, 255),
                panel_tint: ColorRgba::new(49, 50, 68, 255),
                background: ColorRgba::new(17, 17, 27, 248),        // #11111B Crust
                card: ColorRgba::new(30, 30, 46, 255),              // #1E1E2E Base
                muted: ColorRgba::new(49, 50, 68, 255),             // #313244 Surface0
                border: ColorRgba::new(88, 91, 112, 255),           // #585B70 Surface2
                border_highlight: ColorRgba::new(203, 166, 247, 255),// #CBA6F7 Mauve
                primary: ColorRgba::new(203, 166, 247, 255),        // #CBA6F7 Mauve
                primary_fg: ColorRgba::new(17, 17, 27, 255),
                secondary: ColorRgba::new(69, 71, 90, 255),         // #45475A Surface1
                accent: ColorRgba::new(245, 194, 231, 255),         // #F5C2E7 Pink
                destructive: ColorRgba::new(243, 139, 168, 255),    // #F38BA8 Red
                success: ColorRgba::new(166, 227, 161, 255),        // #A6E3A1 Green
                text_main: ColorRgba::new(205, 214, 244, 255),      // #CDD6F4 Text
                text_heading: ColorRgba::new(250, 179, 135, 255),   // #FAB387 Peach
                text_muted: ColorRgba::new(166, 173, 200, 255),     // #A6ADC8 Subtext0
            },
            UiThemeId::TokyoNight => Self {
                id: UiThemeId::TokyoNight,
                name: "Tokyo Night",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(26, 27, 38, 255),
                panel_tint: ColorRgba::new(36, 40, 59, 255),
                background: ColorRgba::new(22, 22, 30, 248),        // #16161E Storm Dark
                card: ColorRgba::new(26, 27, 38, 255),              // #1A1B26 Night BG
                muted: ColorRgba::new(36, 40, 59, 255),             // #24283B Terminal Black
                border: ColorRgba::new(65, 72, 104, 255),           // #414868 Comment
                border_highlight: ColorRgba::new(122, 162, 247, 255),// #7AA2F7 Tokyo Blue
                primary: ColorRgba::new(122, 162, 247, 255),        // #7AA2F7 Blue
                primary_fg: ColorRgba::new(22, 22, 30, 255),
                secondary: ColorRgba::new(59, 66, 97, 255),         // #3B4261
                accent: ColorRgba::new(187, 154, 247, 255),         // #BB9AF7 Magenta
                destructive: ColorRgba::new(247, 118, 142, 255),    // #F7768E Red
                success: ColorRgba::new(158, 206, 106, 255),        // #9ECE6A Green
                text_main: ColorRgba::new(192, 202, 245, 255),      // #C0CAF5 FG
                text_heading: ColorRgba::new(125, 207, 255, 255),   // #7DCFFF Cyan
                text_muted: ColorRgba::new(154, 165, 206, 255),     // #9AA5CE
            },
            UiThemeId::Dracula => Self {
                id: UiThemeId::Dracula,
                name: "Dracula",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(40, 42, 54, 255),
                panel_tint: ColorRgba::new(68, 71, 90, 255),
                background: ColorRgba::new(30, 31, 41, 248),        // #1E1F29 Darkest
                card: ColorRgba::new(40, 42, 54, 255),              // #282A36 Background
                muted: ColorRgba::new(68, 71, 90, 255),             // #44475A Current Line
                border: ColorRgba::new(98, 114, 164, 255),          // #6272A4 Comment
                border_highlight: ColorRgba::new(189, 147, 249, 255),// #BD93F9 Purple
                primary: ColorRgba::new(255, 121, 198, 255),        // #FF79C6 Pink
                primary_fg: ColorRgba::new(40, 42, 54, 255),
                secondary: ColorRgba::new(68, 71, 90, 255),
                accent: ColorRgba::new(139, 233, 253, 255),         // #8BE9FD Cyan
                destructive: ColorRgba::new(255, 85, 85, 255),      // #FF5555 Red
                success: ColorRgba::new(80, 250, 123, 255),         // #50FA7B Green
                text_main: ColorRgba::new(248, 248, 242, 255),      // #F8F8F2 Foreground
                text_heading: ColorRgba::new(189, 147, 249, 255),   // #BD93F9 Purple
                text_muted: ColorRgba::new(139, 233, 253, 255),     // #8BE9FD Cyan
            },
            UiThemeId::Nord => Self {
                id: UiThemeId::Nord,
                name: "Nord Polar",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(46, 52, 64, 255),
                panel_tint: ColorRgba::new(59, 66, 82, 255),
                background: ColorRgba::new(36, 41, 51, 248),        // #242933 Polar Deep
                card: ColorRgba::new(46, 52, 64, 255),              // #2E3440 Nord0
                muted: ColorRgba::new(59, 66, 82, 255),             // #3B4252 Nord1
                border: ColorRgba::new(76, 86, 106, 255),           // #4C566A Nord3
                border_highlight: ColorRgba::new(136, 192, 208, 255),// #88C0D0 Nord8 Frost
                primary: ColorRgba::new(136, 192, 208, 255),        // #88C0D0 Frost Cyan
                primary_fg: ColorRgba::new(46, 52, 64, 255),
                secondary: ColorRgba::new(67, 76, 94, 255),         // #434C5E Nord2
                accent: ColorRgba::new(129, 161, 193, 255),         // #81A1C1 Nord9
                destructive: ColorRgba::new(191, 97, 106, 255),     // #BF616A Nord11
                success: ColorRgba::new(163, 190, 140, 255),        // #A3BE8C Nord14
                text_main: ColorRgba::new(236, 239, 244, 255),      // #ECEFF4 Nord6
                text_heading: ColorRgba::new(143, 188, 187, 255),   // #8FBCBB Nord7
                text_muted: ColorRgba::new(216, 222, 233, 255),     // #D8DEE9 Nord4
            },
            UiThemeId::Cyberpunk => Self {
                id: UiThemeId::Cyberpunk,
                name: "Cyberpunk Neon",
                use_vanilla_9slice: false,
                box_tint: ColorRgba::new(18, 12, 36, 255),
                panel_tint: ColorRgba::new(32, 20, 58, 255),
                background: ColorRgba::new(10, 6, 22, 248),         // #0A0616 Night City
                card: ColorRgba::new(20, 14, 40, 255),              // #140E28
                muted: ColorRgba::new(36, 22, 68, 255),             // #241644
                border: ColorRgba::new(255, 0, 127, 255),           // #FF007F Neon Pink
                border_highlight: ColorRgba::new(0, 240, 255, 255), // #00F0FF Cyber Cyan
                primary: ColorRgba::new(252, 238, 10, 255),         // #FCEE0A Cyber Yellow
                primary_fg: ColorRgba::new(10, 6, 22, 255),
                secondary: ColorRgba::new(62, 28, 108, 255),
                accent: ColorRgba::new(0, 240, 255, 255),           // #00F0FF Cyan
                destructive: ColorRgba::new(255, 0, 85, 255),       // #FF0055
                success: ColorRgba::new(0, 255, 159, 255),          // #00FF9F
                text_main: ColorRgba::new(230, 248, 255, 255),      // #E6F8FF
                text_heading: ColorRgba::new(252, 238, 10, 255),    // #FCEE0A Yellow
                text_muted: ColorRgba::new(0, 240, 255, 255),       // #00F0FF Cyan
            },
        }
    }
}
