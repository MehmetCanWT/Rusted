use crate::dom::{NodeKey, UiTree};
use crate::tailwind::apply_tailwind_classes;

/// GPUI-inspired fluent builder for constructing RustedUI trees programmatically.
pub struct DivBuilder<'a> {
    pub tree: &'a mut UiTree,
    pub key: NodeKey,
}

impl<'a> DivBuilder<'a> {
    pub fn new(tree: &'a mut UiTree, tag: &str) -> Self {
        let key = tree.create_element(tag, "");
        Self { tree, key }
    }

    #[inline]
    pub fn id(self, id: &str) -> Self {
        self.tree.set_element_id(self.key, id);
        self
    }

    #[inline]
    pub fn class(self, classes: &str) -> Self {
        if let Some(node) = self.tree.arena.get_mut(self.key) {
            apply_tailwind_classes(classes, &mut node.style, &mut node.flags, &mut node.visual);
            let _ = self.tree.taffy.set_style(node.taffy_id, node.style.clone());
        }
        self
    }

    #[inline]
    pub fn flex(self) -> Self {
        self.class("flex")
    }

    #[inline]
    pub fn flex_col(self) -> Self {
        self.class("flex-col")
    }

    #[inline]
    pub fn flex_row(self) -> Self {
        self.class("flex-row")
    }

    #[inline]
    pub fn items_center(self) -> Self {
        self.class("items-center")
    }

    #[inline]
    pub fn justify_between(self) -> Self {
        self.class("justify-between")
    }

    #[inline]
    pub fn justify_center(self) -> Self {
        self.class("justify-center")
    }

    #[inline]
    pub fn gap(self, units: u32) -> Self {
        self.class(&format!("gap-{units}"))
    }

    #[inline]
    pub fn p(self, units: u32) -> Self {
        self.class(&format!("p-{units}"))
    }

    #[inline]
    pub fn sdv_box(self) -> Self {
        self.class("sdv-box")
    }

    #[inline]
    pub fn sdv_panel(self) -> Self {
        self.class("sdv-panel")
    }

    #[inline]
    pub fn sdv_parchment(self) -> Self {
        self.class("sdv-parchment")
    }

    #[inline]
    pub fn sdv_btn(self) -> Self {
        self.class("sdv-btn")
    }

    #[inline]
    pub fn on_click(self, action_name: &str) -> Self {
        if let Some(node) = self.tree.arena.get_mut(self.key) {
            node.action_name = Some(action_name.to_string());
        }
        self
    }

    #[inline]
    pub fn text(self, content: &str) -> Self {
        self.tree.set_node_text(self.key, content);
        self
    }

    #[inline]
    pub fn child(self, child_key: NodeKey) -> Self {
        self.tree.append_child(self.key, child_key);
        self
    }

    #[inline]
    pub fn build(self) -> NodeKey {
        self.key
    }
}
