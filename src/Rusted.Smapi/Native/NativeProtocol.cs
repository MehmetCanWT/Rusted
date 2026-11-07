using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace Rusted.Smapi.Native
{
    public enum RenderCommandKind : uint
    {
        DrawNineSlice = 0,
        DrawSprite = 1,
        DrawText = 2,
        DrawRect = 3,
        PushClipRect = 4,
        PopClipRect = 5
    }

    public enum TextureId : uint
    {
        None = 0,
        Cursors = 1,
        Cursors2 = 2,
        MenuTiles = 3,
        MenuTilesUncolored = 4,
        TextBox = 5,
        Objects = 6,
        LucideAtlas = 7,
        Custom = 255
    }

    public enum UiThemeId : uint
    {
        PelicanClassic = 0,
        ShadcnZincDark = 1,
        WizardArcane = 2,
        JojaCorp = 3,
        JunimoForest = 4,
        CatppuccinMocha = 5,
        TokyoNight = 6,
        Dracula = 7,
        Nord = 8,
        Cyberpunk = 9
    }

    public enum VanillaFontId : uint
    {
        Small = 0,
        Dialogue = 1,
        Tiny = 2
    }

    public enum UiEventKind : uint
    {
        Click = 0,
        HoverEnter = 1,
        HoverLeave = 2,
        ValueChanged = 3,
        CloseRequested = 4,
        SoundCue = 5,
        FocusInput = 6
    }

    public enum SoundCueId : uint
    {
        None = 0,
        Shiny4 = 1,
        BigSelect = 2,
        SmallSelect = 3,
        Dwop = 4,
        Coin = 5,
        Drumkit6 = 6,
        Cancel = 7,
        Trashcan = 8
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct RectF
    {
        public float X;
        public float Y;
        public float Width;
        public float Height;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct RectI
    {
        public int X;
        public int Y;
        public int Width;
        public int Height;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ColorRgba
    {
        public byte R;
        public byte G;
        public byte B;
        public byte A;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct BorderInsets
    {
        public float Top;
        public float Right;
        public float Bottom;
        public float Left;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct RenderCommand
    {
        public uint Kind;
        public uint TextureId;
        public uint FontId;
        public float Scale;
        public RectF DestRect;
        public RectI SourceRect;
        public BorderInsets Insets;
        public ColorRgba Color;
        public ColorRgba ShadowColor;
        public float ShadowOffsetX;
        public float ShadowOffsetY;
        public uint TextOffset;
        public uint TextLen;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct UiEvent
    {
        public ulong NodeKeyBits;
        public uint Kind;
        public uint SoundCue;
        public uint ActionOffset;
        public uint ActionLen;
        public float MouseX;
        public float MouseY;
        public float FloatValue;
        public int IntValue;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct NativeGlyphMetric
    {
        public uint Character;
        public float Advance;
        public float LeftBearing;
        public float RightBearing;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct RenderSliceView
    {
        public RenderCommand* CommandsPtr;
        public UIntPtr CommandsLen;
        public byte* TextPoolPtr;
        public UIntPtr TextPoolLen;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct EventSliceView
    {
        public UiEvent* EventsPtr;
        public UIntPtr EventsLen;
        public byte* TextPoolPtr;
        public UIntPtr TextPoolLen;
    }

    public static unsafe class NativeMethods
    {
        public const string LibName = "rusted_bridge";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern IntPtr rusted_tree_create();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_destroy(IntPtr tree);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_clear(IntPtr tree);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_set_root(IntPtr tree, ulong rootBits);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void rusted_tree_sync_font(
            IntPtr tree,
            uint fontId,
            float lineSpacing,
            float spacing,
            NativeGlyphMetric* glyphsPtr,
            uint glyphCount);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong rusted_tree_load_markup(
            IntPtr tree,
            byte* utf8Ptr,
            uint utf8Len);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern ulong rusted_node_create(
            IntPtr tree,
            byte* tagPtr,
            uint tagLen,
            byte* classPtr,
            uint classLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_node_append_child(
            IntPtr tree,
            ulong parentBits,
            ulong childBits);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_node_set_id(
            IntPtr tree,
            ulong nodeBits,
            byte* idPtr,
            uint idLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_node_set_action(
            IntPtr tree,
            ulong nodeBits,
            byte* actPtr,
            uint actLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_node_set_text(
            IntPtr tree,
            ulong nodeBits,
            byte* textPtr,
            uint textLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_text_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            byte* textPtr,
            uint textLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_progress_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            float value,
            float max);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_checked_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            uint isChecked);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_visible_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            uint isVisible);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_slot_item_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            int itemIndex,
            int count,
            uint quality);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_hearts_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            int filled,
            int max);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_set_tooltip_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            byte* tipPtr,
            uint tipLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_tree_set_theme(IntPtr tree, uint themeId);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern ulong rusted_tree_memory_bytes(IntPtr tree);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_uses_global_theme(IntPtr tree);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern void rusted_node_set_icon(
            IntPtr tree,
            ulong nodeBits,
            byte* iconPtr,
            uint iconLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_scroll_by_id(
            IntPtr tree,
            byte* idPtr,
            uint idLen,
            float deltaY);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_update_input(
            IntPtr tree,
            float mouseX,
            float mouseY,
            uint mouseDown,
            float scrollDelta,
            EventSliceView* outView);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [SuppressGCTransition]
        public static extern uint rusted_tree_render(
            IntPtr tree,
            float viewportW,
            float viewportH,
            RenderSliceView* outView);
    }
}
