use std::collections::{HashMap, HashSet};
use crate::types::{NativeGlyphMetric, VanillaFont};

#[derive(Clone, Debug)]
pub struct FontAtlas {
    pub font_kind: VanillaFont,
    pub line_spacing: f32,
    pub spacing: f32,
    pub ascii_advance: [f32; 128],
    pub unicode_advance: HashMap<char, f32>,
    pub supported_chars: HashSet<char>,
    pub has_synced: bool,
    pub default_advance: f32,
}

impl FontAtlas {
    pub fn new_default(font_kind: VanillaFont) -> Self {
        let (line_spacing, default_adv, spacing) = match font_kind {
            VanillaFont::Small => (32.0, 14.5, 0.0),
            VanillaFont::Dialogue => (45.0, 23.0, 0.0),
            VanillaFont::Tiny => (20.0, 10.0, 0.0),
        };

        let mut ascii_advance = [default_adv; 128];
        ascii_advance[' ' as usize] = default_adv * 0.6;
        ascii_advance['i' as usize] = default_adv * 0.55;
        ascii_advance['l' as usize] = default_adv * 0.55;
        ascii_advance['I' as usize] = default_adv * 0.65;
        ascii_advance['.' as usize] = default_adv * 0.5;
        ascii_advance[',' as usize] = default_adv * 0.5;
        ascii_advance[':' as usize] = default_adv * 0.5;
        ascii_advance['!' as usize] = default_adv * 0.55;
        ascii_advance['m' as usize] = default_adv * 1.3;
        ascii_advance['w' as usize] = default_adv * 1.3;
        ascii_advance['M' as usize] = default_adv * 1.35;
        ascii_advance['W' as usize] = default_adv * 1.35;

        Self {
            font_kind,
            line_spacing,
            spacing,
            ascii_advance,
            unicode_advance: HashMap::new(),
            supported_chars: HashSet::new(),
            has_synced: false,
            default_advance: default_adv,
        }
    }

    pub fn sync_from_native(
        &mut self,
        line_spacing: f32,
        spacing: f32,
        glyphs: &[NativeGlyphMetric],
    ) {
        if line_spacing > 0.0 {
            self.line_spacing = line_spacing;
        }
        self.spacing = spacing;
        self.supported_chars.clear();
        self.has_synced = true;

        for g in glyphs {
            let total_adv = (g.left_bearing + g.advance + g.right_bearing + spacing).max(1.0);
            if let Some(ch) = char::from_u32(g.character) {
                self.supported_chars.insert(ch);
                if g.character < 128 {
                    self.ascii_advance[g.character as usize] = total_adv;
                } else {
                    self.unicode_advance.insert(ch, total_adv);
                }
            }
        }
    }

    /// Normalizes characters missing from the currently loaded SpriteFont (e.g. Turkish ş, ğ, ı, İ when running English locale).
    pub fn sanitize_text(&self, text: &str) -> String {
        if !self.has_synced {
            return text.to_string();
        }
        let mut out = String::with_capacity(text.len());
        for ch in text.chars() {
            if ch == '\n' || ch == '\t' || (ch as u32) < 128 || self.supported_chars.contains(&ch) {
                out.push(ch);
            } else {
                match ch {
                    'ş' => out.push('s'),
                    'Ş' => out.push('S'),
                    'ğ' => out.push('g'),
                    'Ğ' => out.push('G'),
                    'ı' => out.push('i'),
                    'İ' => out.push('I'),
                    '—' | '–' => out.push('-'),
                    _ => out.push(ch),
                }
            }
        }
        out
    }

    #[inline(always)]
    pub fn char_advance(&self, ch: char, scale: f32) -> f32 {
        let code = ch as u32;
        let base = if code < 128 {
            self.ascii_advance[code as usize]
        } else {
            self.unicode_advance.get(&ch).copied().unwrap_or(self.default_advance)
        };
        base * scale
    }

    /// Computes minimum width required so no single word is broken mid-word.
    pub fn measure_min_content_width(&self, text: &str, scale: f32) -> f32 {
        let clean = self.sanitize_text(text);
        let mut max_word_w = 0.0f32;
        for word in clean.split_whitespace() {
            let w: f32 = word.chars().map(|c| self.char_advance(c, scale)).sum();
            max_word_w = max_word_w.max(w);
        }
        (max_word_w + 4.0).ceil()
    }

    /// Zero-allocation measurement for Taffy leaf layout.
    pub fn measure_text(&self, text: &str, max_width: f32, scale: f32) -> (f32, f32) {
        if text.is_empty() {
            return (0.0, 0.0);
        }

        let clean = self.sanitize_text(text);
        let mut max_observed_width: f32 = 0.0;
        let mut current_line_width: f32 = 0.0;
        let mut current_word_width: f32 = 0.0;
        let mut line_count: u32 = 1;
        let space_adv = self.char_advance(' ', scale);

        for ch in clean.chars() {
            match ch {
                '\n' => {
                    max_observed_width = max_observed_width.max(current_line_width + current_word_width);
                    current_line_width = 0.0;
                    current_word_width = 0.0;
                    line_count += 1;
                }
                ' ' | '\t' => {
                    if current_line_width + current_word_width > max_width && current_line_width > 0.0 {
                        max_observed_width = max_observed_width.max(current_line_width);
                        current_line_width = current_word_width + space_adv;
                        line_count += 1;
                    } else {
                        current_line_width += current_word_width + space_adv;
                    }
                    current_word_width = 0.0;
                }
                _ => {
                    current_word_width += self.char_advance(ch, scale);
                }
            }
        }

        if current_line_width + current_word_width > max_width && current_line_width > 0.0 {
            max_observed_width = max_observed_width.max(current_line_width);
            max_observed_width = max_observed_width.max(current_word_width);
            line_count += 1;
        } else {
            max_observed_width = max_observed_width.max(current_line_width + current_word_width);
        }

        let total_height = (line_count as f32) * self.line_spacing * scale;
        ((max_observed_width + 4.0).ceil(), total_height.ceil())
    }

    /// Invokes `callback(line_str, line_index, line_y_offset)` for each wrapped line.
    pub fn for_each_wrapped_line<F>(&self, text: &str, max_width: f32, scale: f32, mut callback: F)
    where
        F: FnMut(&str, u32, f32),
    {
        if text.is_empty() {
            return;
        }

        let clean = self.sanitize_text(text);
        let line_h = self.line_spacing * scale;
        let mut line_idx = 0u32;

        for raw_line in clean.split('\n') {
            if max_width.is_infinite() || max_width <= 0.0 {
                callback(raw_line, line_idx, line_idx as f32 * line_h);
                line_idx += 1;
                continue;
            }

            let mut line_start_byte = 0usize;
            let mut line_end_byte = 0usize;
            let mut current_w = 0.0f32;
            let space_adv = self.char_advance(' ', scale);

            for word in raw_line.split(' ') {
                let word_w: f32 = word.chars().map(|c| self.char_advance(c, scale)).sum();
                let word_ptr_offset = unsafe { word.as_ptr().offset_from(raw_line.as_ptr()) as usize };

                if current_w > 0.0 && current_w + space_adv + word_w > max_width {
                    callback(&raw_line[line_start_byte..line_end_byte], line_idx, line_idx as f32 * line_h);
                    line_idx += 1;
                    line_start_byte = word_ptr_offset;
                    line_end_byte = word_ptr_offset + word.len();
                    current_w = word_w;
                } else {
                    if current_w == 0.0 {
                        line_start_byte = word_ptr_offset;
                        current_w = word_w;
                    } else {
                        current_w += space_adv + word_w;
                    }
                    line_end_byte = word_ptr_offset + word.len();
                }
            }

            if line_end_byte >= line_start_byte {
                callback(&raw_line[line_start_byte..line_end_byte], line_idx, line_idx as f32 * line_h);
                line_idx += 1;
            }
        }
    }
}
