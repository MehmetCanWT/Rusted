use std::slice;
use rusted_core::{
    EventSliceView, NativeGlyphMetric, NodeKey, RenderSliceView, UiTree,
};

#[no_mangle]
pub extern "C" fn rusted_tree_create() -> *mut UiTree {
    Box::into_raw(Box::new(UiTree::new()))
}

/// # Safety
/// `tree` must be a valid pointer returned by `rusted_tree_create` or null.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_destroy(tree: *mut UiTree) {
    if !tree.is_null() {
        drop(Box::from_raw(tree));
    }
}

/// # Safety
/// `tree` must be valid, and `glyphs_ptr` must point to `glyph_count` elements of `NativeGlyphMetric`.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_sync_font(
    tree: *mut UiTree,
    font_id: u32,
    line_spacing: f32,
    spacing: f32,
    glyphs_ptr: *const NativeGlyphMetric,
    glyph_count: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    if glyphs_ptr.is_null() || glyph_count == 0 {
        return;
    }
    let glyphs = slice::from_raw_parts(glyphs_ptr, glyph_count as usize);
    t.sync_font(font_id, line_spacing, spacing, glyphs);
}

unsafe fn str_from_utf8_raw<'a>(ptr: *const u8, len: u32) -> &'a str {
    if ptr.is_null() || len == 0 {
        return "";
    }
    let bytes = slice::from_raw_parts(ptr, len as usize);
    std::str::from_utf8(bytes).unwrap_or("")
}

/// Compiles HTML/Tailwind markup into the retained UI tree. Returns root `NodeKey` bits (or 0 on error).
/// # Safety
/// `tree` and `utf8_ptr` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_load_markup(
    tree: *mut UiTree,
    utf8_ptr: *const u8,
    utf8_len: u32,
) -> u64 {
    let Some(t) = tree.as_mut() else { return 0 };
    let markup = str_from_utf8_raw(utf8_ptr, utf8_len);
    match t.load_template_markup(markup) {
        Ok(root_key) => root_key.to_bits(),
        Err(_) => 0,
    }
}

/// Creates a node programmatically (for GPUI-style C# Fluent Builder).
/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_create(
    tree: *mut UiTree,
    tag_ptr: *const u8,
    tag_len: u32,
    class_ptr: *const u8,
    class_len: u32,
) -> u64 {
    let Some(t) = tree.as_mut() else { return 0 };
    let tag = str_from_utf8_raw(tag_ptr, tag_len);
    let classes = str_from_utf8_raw(class_ptr, class_len);
    t.create_element(tag, classes).to_bits()
}

/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_clear(tree: *mut UiTree) {
    let Some(t) = tree.as_mut() else { return };
    t.clear_tree();
}

/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_root(tree: *mut UiTree, root_bits: u64) {
    let Some(t) = tree.as_mut() else { return };
    if root_bits != 0 {
        t.root_key = Some(NodeKey::from_bits(root_bits));
    }
}

/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_append_child(
    tree: *mut UiTree,
    parent_bits: u64,
    child_bits: u64,
) {
    let Some(t) = tree.as_mut() else { return };
    let parent = NodeKey::from_bits(parent_bits);
    let child = NodeKey::from_bits(child_bits);
    if t.arena.contains_key(parent) && t.arena.contains_key(child) {
        t.append_child(parent, child);
    }
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_set_id(
    tree: *mut UiTree,
    node_bits: u64,
    id_ptr: *const u8,
    id_len: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    let key = NodeKey::from_bits(node_bits);
    let id = str_from_utf8_raw(id_ptr, id_len);
    t.set_element_id(key, id);
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_set_action(
    tree: *mut UiTree,
    node_bits: u64,
    act_ptr: *const u8,
    act_len: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    let key = NodeKey::from_bits(node_bits);
    let act = str_from_utf8_raw(act_ptr, act_len);
    if let Some(n) = t.arena.get_mut(key) {
        n.action_name = Some(act.to_string());
    }
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_set_text(
    tree: *mut UiTree,
    node_bits: u64,
    text_ptr: *const u8,
    text_len: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    let key = NodeKey::from_bits(node_bits);
    let text = str_from_utf8_raw(text_ptr, text_len);
    t.set_node_text(key, text);
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_text_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    text_ptr: *const u8,
    text_len: u32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    let text = str_from_utf8_raw(text_ptr, text_len);
    u32::from(t.set_text_by_id(id, text))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_progress_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    value: f32,
    max: f32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.set_progress_by_id(id, value, max))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_checked_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    checked: u32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.set_checked_by_id(id, checked != 0))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_visible_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    visible: u32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.set_visible_by_id(id, visible != 0))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_slot_item_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    item_index: i32,
    count: i32,
    quality: u32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.set_slot_item_by_id(id, item_index, count, quality as u8))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_hearts_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    filled: i32,
    max: i32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.set_hearts_by_id(id, filled, max))
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_tooltip_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    tip_ptr: *const u8,
    tip_len: u32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    let tip = str_from_utf8_raw(tip_ptr, tip_len);
    u32::from(t.set_tooltip_by_id(id, tip))
}

/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_set_theme(tree: *mut UiTree, theme_id: u32) {
    let Some(t) = tree.as_mut() else { return };
    t.set_theme(theme_id);
}

/// Returns the native Rust memory footprint (in bytes) of the `UiTree`.
/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_memory_bytes(tree: *mut UiTree) -> u64 {
    let Some(t) = tree.as_ref() else { return 0 };
    t.memory_usage_bytes() as u64
}

/// Returns 1 if the root XML template explicitly specified `use-rusted-theme="true"`, 0 otherwise.
/// # Safety
/// `tree` must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_uses_global_theme(tree: *mut UiTree) -> u32 {
    let Some(t) = tree.as_ref() else { return 0 };
    u32::from(t.use_global_theme)
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_node_set_icon(
    tree: *mut UiTree,
    node_bits: u64,
    icon_ptr: *const u8,
    icon_len: u32,
) {
    let Some(t) = tree.as_mut() else { return };
    let key = NodeKey::from_bits(node_bits);
    let icon = str_from_utf8_raw(icon_ptr, icon_len);
    if let Some(n) = t.arena.get_mut(key) {
        n.visual.icon_name = Some(icon.to_string());
        n.flags.insert(rusted_core::ComponentFlags::LUCIDE_ICON);
    }
}

/// # Safety
/// `tree` and string pointers must be valid.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_scroll_by_id(
    tree: *mut UiTree,
    id_ptr: *const u8,
    id_len: u32,
    delta_y: f32,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let id = str_from_utf8_raw(id_ptr, id_len);
    u32::from(t.scroll_by_id(id, delta_y))
}

/// Updates mouse input and fills `out_view` with the pull-based `UiEvent` queue.
/// Returns 1 if the cursor is currently over any UI element (so C# can suppress game world clicks), 0 otherwise.
/// # Safety
/// `tree` and `out_view` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_update_input(
    tree: *mut UiTree,
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: u32,
    scroll_delta: f32,
    out_view: *mut EventSliceView,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let hit = t.update_input(mouse_x, mouse_y, mouse_down != 0, scroll_delta);
    if !out_view.is_null() {
        *out_view = t.current_event_view();
    }
    u32::from(hit)
}

/// Computes layout and populates `out_view` with the contiguous zero-copy `RenderCommand` buffer.
/// Returns the number of commands generated.
/// # Safety
/// `tree` and `out_view` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn rusted_tree_render(
    tree: *mut UiTree,
    viewport_w: f32,
    viewport_h: f32,
    out_view: *mut RenderSliceView,
) -> u32 {
    let Some(t) = tree.as_mut() else { return 0 };
    let view = t.compute_and_render(viewport_w, viewport_h);
    let len = view.commands_len as u32;
    if !out_view.is_null() {
        *out_view = view;
    }
    len
}
