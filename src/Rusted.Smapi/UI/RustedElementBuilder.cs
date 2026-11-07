using System;
using System.Text;
using Rusted.Smapi.Native;

namespace Rusted.Smapi.UI
{
    public readonly struct RustedContext
    {
        internal readonly IntPtr Tree;

        internal RustedContext(IntPtr tree) => Tree = tree;

        public RustedNode Div(string classes = "") => Create("div", classes);
        public RustedNode Panel(string classes = "") => Create("panel", "sdv-panel " + classes);
        public RustedNode Banner(string text, string classes = "") => Create("banner", "sdv-parchment " + classes).Text(text);
        public RustedNode Text(string content, string classes = "font-small text-sdv-dark") => Create("span", classes).Text(content);
        public RustedNode Title(string content, string classes = "font-dialogue text-sdv-brown") => Create("span", classes).Text(content);
        public RustedNode Button(string id, string label, string action, string classes = "") =>
            Create("button", "sdv-btn " + classes).Id(id).Text(label).OnClick(action);
        public RustedNode CloseButton(string id = "btn_close") => Create("close-button", "sdv-close").Id(id);
        public RustedNode Divider() => Create("hr", "sdv-divider");
        public RustedNode ProgressBar(string id, float value, float max = 100f, string classes = "w-full h-6")
        {
            var node = Create("progress-bar", "sdv-bar " + classes).Id(id);
            SetProgress(id, value, max);
            return node;
        }
        public RustedNode ItemSlot(string id, int itemIndex, int count, uint quality = 0, string action = "")
        {
            var node = Create("item-slot", "sdv-slot").Id(id);
            if (!string.IsNullOrEmpty(action))
                node.OnClick(action);
            SetSlot(id, itemIndex, count, quality);
            return node;
        }
        public RustedNode Checkbox(string id, bool isChecked, string action)
        {
            var node = Create("checkbox", "sdv-checkbox").Id(id).OnClick(action);
            SetChecked(id, isChecked);
            return node;
        }
        public RustedNode Slider(string id, float value, float max, string action, string classes = "w-full")
        {
            var node = Create("slider", "sdv-slider " + classes).Id(id).OnClick(action);
            SetProgress(id, value, max);
            return node;
        }
        public RustedNode TextInput(string id, string text, string action, string classes = "w-full") =>
            Create("text-input", "sdv-input " + classes).Id(id).Text(text).OnClick(action);
        public RustedNode Tab(string id, string label, bool isActive, string action)
        {
            var node = Create("tab", "sdv-tab").Id(id).Text(label).OnClick(action);
            SetChecked(id, isActive);
            return node;
        }
        public RustedNode ScrollView(string id, string classes = "w-full h-180") =>
            Create("scroll-view", "sdv-scroll " + classes).Id(id);
        public RustedNode ArrowLeft(string id, string action) =>
            Create("arrow-left", "sdv-arrow-left").Id(id).OnClick(action);
        public RustedNode ArrowRight(string id, string action) =>
            Create("arrow-right", "sdv-arrow-right").Id(id).OnClick(action);
        public RustedNode ArrowUp(string id, string action) =>
            Create("arrow-up", "sdv-arrow-up").Id(id).OnClick(action);
        public RustedNode ArrowDown(string id, string action) =>
            Create("arrow-down", "sdv-arrow-down").Id(id).OnClick(action);
        public RustedNode TrashButton(string id, string action) =>
            Create("trash-button", "sdv-trash").Id(id).OnClick(action);
        public RustedNode HeartRating(string id, int filled, int max = 10)
        {
            var node = Create("heart-rating", "sdv-hearts").Id(id);
            SetHearts(id, filled, max);
            return node;
        }
        public RustedNode CoinIcon() => Create("coin-icon", "sdv-coin");
        public RustedNode Card(string classes = "") => Create("card", "shadcn-card " + classes);
        public RustedNode Badge(string label, string variantClass = "badge-primary", string classes = "") =>
            Create("badge", "sdv-badge " + variantClass + " " + classes).Text(label);
        public RustedNode Switch(string id, bool isChecked, string action)
        {
            var node = Create("switch", "sdv-switch").Id(id).OnClick(action);
            SetChecked(id, isChecked);
            return node;
        }
        public unsafe RustedNode Icon(string iconName, string classes = "lucide-md")
        {
            var node = Create("icon", "lucide-icon " + classes);
            byte[] bytes = Encoding.UTF8.GetBytes(iconName);
            fixed (byte* ptr = bytes)
            {
                NativeMethods.rusted_node_set_icon(Tree, node.KeyBits, ptr, (uint)bytes.Length);
            }
            return node;
        }

        private unsafe RustedNode Create(string tag, string classes)
        {
            byte[] tagBytes = Encoding.UTF8.GetBytes(tag);
            byte[] clsBytes = Encoding.UTF8.GetBytes(classes);
            fixed (byte* tPtr = tagBytes)
            fixed (byte* cPtr = clsBytes)
            {
                ulong bits = NativeMethods.rusted_node_create(Tree, tPtr, (uint)tagBytes.Length, cPtr, (uint)clsBytes.Length);
                return new RustedNode(Tree, bits);
            }
        }

        private unsafe void SetProgress(string id, float value, float max)
        {
            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* ptr = idBytes)
            {
                NativeMethods.rusted_tree_set_progress_by_id(Tree, ptr, (uint)idBytes.Length, value, max);
            }
        }

        private unsafe void SetSlot(string id, int itemIndex, int count, uint quality)
        {
            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* ptr = idBytes)
            {
                NativeMethods.rusted_tree_set_slot_item_by_id(Tree, ptr, (uint)idBytes.Length, itemIndex, count, quality);
            }
        }

        private unsafe void SetChecked(string id, bool isChecked)
        {
            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* ptr = idBytes)
            {
                NativeMethods.rusted_tree_set_checked_by_id(Tree, ptr, (uint)idBytes.Length, isChecked ? 1u : 0u);
            }
        }

        private unsafe void SetHearts(string id, int filled, int max)
        {
            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* ptr = idBytes)
            {
                NativeMethods.rusted_tree_set_hearts_by_id(Tree, ptr, (uint)idBytes.Length, filled, max);
            }
        }
    }

    /// <summary>
    /// GPUI-inspired chainable UI element handle.
    /// </summary>
    public readonly struct RustedNode
    {
        internal readonly IntPtr Tree;
        public readonly ulong KeyBits;

        internal RustedNode(IntPtr tree, ulong keyBits)
        {
            Tree = tree;
            KeyBits = keyBits;
        }

        public unsafe RustedNode Id(string id)
        {
            byte[] bytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* ptr = bytes)
            {
                NativeMethods.rusted_node_set_id(Tree, KeyBits, ptr, (uint)bytes.Length);
            }
            return this;
        }

        public unsafe RustedNode OnClick(string actionName)
        {
            byte[] bytes = Encoding.UTF8.GetBytes(actionName);
            fixed (byte* ptr = bytes)
            {
                NativeMethods.rusted_node_set_action(Tree, KeyBits, ptr, (uint)bytes.Length);
            }
            return this;
        }

        public unsafe RustedNode Text(string content)
        {
            byte[] bytes = Encoding.UTF8.GetBytes(content);
            fixed (byte* ptr = bytes)
            {
                NativeMethods.rusted_node_set_text(Tree, KeyBits, ptr, (uint)bytes.Length);
            }
            return this;
        }

        public RustedNode Children(params RustedNode[] children)
        {
            for (int i = 0; i < children.Length; i++)
            {
                NativeMethods.rusted_node_append_child(Tree, KeyBits, children[i].KeyBits);
            }
            return this;
        }
    }
}
