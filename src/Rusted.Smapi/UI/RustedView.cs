using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using Microsoft.Xna.Framework.Graphics;
using Rusted.Smapi.Native;
using Rusted.Smapi.Rendering;
using StardewValley;

namespace Rusted.Smapi.UI
{
    public sealed class RustedView : IDisposable
    {
        private IntPtr _tree;
        private readonly RustedRenderer _renderer = new();
        private readonly Dictionary<string, Action<UiEvent>> _actionHandlers = new(StringComparer.OrdinalIgnoreCase);
        private FileSystemWatcher? _hotReloadWatcher;
        private string? _watchedFilePath;
        private volatile bool _reloadPending;

        public IntPtr Handle => _tree;
        public string OwnerModId { get; set; } = "Mehmetcan.RustedUI";
        public bool UseRustedUiTheme { get; set; }
        public ulong NativeMemoryBytes => _tree != IntPtr.Zero ? NativeMethods.rusted_tree_memory_bytes(_tree) : 0;
        public int LastCommandCount { get; private set; }
        public double LastRenderMilliseconds { get; private set; }
        public bool SoundEnabled { get; set; } = true;
        public string? FocusedInputId { get; private set; }
        public event Action? OnCloseRequested;
        public event Action? OnHotReloaded;

        public RustedView(string ownerModId = "Mehmetcan.RustedUI", bool useRustedUiTheme = false)
        {
            OwnerModId = ownerModId;
            UseRustedUiTheme = useRustedUiTheme;
            _tree = NativeMethods.rusted_tree_create();
            RustedRenderer.SyncFontsToTree(_tree);
        }

        public void SyncFonts()
        {
            if (_tree != IntPtr.Zero)
                RustedRenderer.SyncFontsToTree(_tree);
        }

        public unsafe void LoadMarkup(string markup)
        {
            if (_tree == IntPtr.Zero || string.IsNullOrEmpty(markup))
                return;

            byte[] utf8 = Encoding.UTF8.GetBytes(markup);
            fixed (byte* ptr = utf8)
            {
                NativeMethods.rusted_tree_load_markup(_tree, ptr, (uint)utf8.Length);
            }
        }

        public void LoadMarkupFileWithHotReload(string filePath)
        {
            _watchedFilePath = filePath;
            if (File.Exists(filePath))
            {
                LoadMarkup(File.ReadAllText(filePath, Encoding.UTF8));
            }

            _hotReloadWatcher?.Dispose();
            string? dir = Path.GetDirectoryName(filePath);
            string file = Path.GetFileName(filePath);
            if (!string.IsNullOrEmpty(dir) && Directory.Exists(dir))
            {
                _hotReloadWatcher = new FileSystemWatcher(dir, file)
                {
                    NotifyFilter = NotifyFilters.LastWrite | NotifyFilters.Size | NotifyFilters.FileName,
                    EnableRaisingEvents = true
                };
                _hotReloadWatcher.Changed += (_, _) => _reloadPending = true;
                _hotReloadWatcher.Created += (_, _) => _reloadPending = true;
            }
        }

        public void BuildWithFluent(Func<RustedContext, RustedNode> builder)
        {
            if (_tree == IntPtr.Zero)
                return;

            _hotReloadWatcher?.Dispose();
            _hotReloadWatcher = null;

            NativeMethods.rusted_tree_clear(_tree);
            var ctx = new RustedContext(_tree);
            RustedNode root = builder(ctx);
            NativeMethods.rusted_tree_set_root(_tree, root.KeyBits);
        }

        public RustedView OnAction(string actionOrId, Action<UiEvent> handler)
        {
            _actionHandlers[actionOrId] = handler;
            return this;
        }

        public RustedView OnAction(string actionOrId, Action handler)
        {
            _actionHandlers[actionOrId] = _ => handler();
            return this;
        }

        public unsafe void SetText(string id, string text)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            byte[] textBytes = Encoding.UTF8.GetBytes(text);
            fixed (byte* idPtr = idBytes)
            fixed (byte* txtPtr = textBytes)
            {
                NativeMethods.rusted_tree_set_text_by_id(_tree, idPtr, (uint)idBytes.Length, txtPtr, (uint)textBytes.Length);
            }
        }

        public unsafe void SetProgress(string id, float value, float max = 100f)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_set_progress_by_id(_tree, idPtr, (uint)idBytes.Length, value, max);
            }
        }

        public unsafe void SetChecked(string id, bool isChecked)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_set_checked_by_id(_tree, idPtr, (uint)idBytes.Length, isChecked ? 1u : 0u);
            }
        }

        public unsafe void SetVisible(string id, bool isVisible)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_set_visible_by_id(_tree, idPtr, (uint)idBytes.Length, isVisible ? 1u : 0u);
            }
        }

        public unsafe void SetSlotItem(string id, int itemIndex, int count, uint quality = 0)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_set_slot_item_by_id(_tree, idPtr, (uint)idBytes.Length, itemIndex, count, quality);
            }
        }

        public unsafe void SetHearts(string id, int filled, int max = 10)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_set_hearts_by_id(_tree, idPtr, (uint)idBytes.Length, filled, max);
            }
        }

        public unsafe void SetTooltip(string id, string tooltip)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            byte[] tipBytes = Encoding.UTF8.GetBytes(tooltip);
            fixed (byte* idPtr = idBytes)
            fixed (byte* tipPtr = tipBytes)
            {
                NativeMethods.rusted_tree_set_tooltip_by_id(_tree, idPtr, (uint)idBytes.Length, tipPtr, (uint)tipBytes.Length);
            }
        }

        public unsafe void ScrollBy(string id, float deltaY)
        {
            if (_tree == IntPtr.Zero)
                return;

            byte[] idBytes = Encoding.UTF8.GetBytes(id);
            fixed (byte* idPtr = idBytes)
            {
                NativeMethods.rusted_tree_scroll_by_id(_tree, idPtr, (uint)idBytes.Length, deltaY);
            }
        }

        public void SetTheme(UiThemeId theme) => SetTheme((uint)theme);

        public void SetTheme(uint themeId)
        {
            if (_tree == IntPtr.Zero)
                return;

            NativeMethods.rusted_tree_set_theme(_tree, themeId);
        }

        public void ApplyGlobalRustedTheme(UiThemeId globalTheme)
        {
            if (_tree == IntPtr.Zero)
                return;

            bool isMainRustedMod = string.Equals(OwnerModId, "Mehmetcan.RustedUI", StringComparison.OrdinalIgnoreCase);
            bool xmlOptedIn = NativeMethods.rusted_tree_uses_global_theme(_tree) != 0;
            if (isMainRustedMod || UseRustedUiTheme || xmlOptedIn)
            {
                SetTheme(globalTheme);
            }
        }

        public unsafe bool UpdateInput(float uiMouseX, float uiMouseY, bool leftMouseDown, float scrollDelta = 0f)
        {
            if (_tree == IntPtr.Zero)
                return false;

            if (_reloadPending && _watchedFilePath != null && File.Exists(_watchedFilePath))
            {
                try
                {
                    string content = File.ReadAllText(_watchedFilePath, Encoding.UTF8);
                    LoadMarkup(content);
                    _reloadPending = false;
                    OnHotReloaded?.Invoke();
                }
                catch
                {
                    // File still being written by editor; retry on next tick
                }
            }

            EventSliceView view;
            uint isOverUi = NativeMethods.rusted_tree_update_input(
                _tree,
                uiMouseX,
                uiMouseY,
                leftMouseDown ? 1u : 0u,
                scrollDelta,
                &view);

            if (view.EventsLen != UIntPtr.Zero && view.EventsPtr != null)
            {
                ReadOnlySpan<UiEvent> events = new(view.EventsPtr, (int)view.EventsLen);
                ReadOnlySpan<byte> pool = view.TextPoolPtr != null
                    ? new ReadOnlySpan<byte>(view.TextPoolPtr, (int)view.TextPoolLen)
                    : ReadOnlySpan<byte>.Empty;

                for (int i = 0; i < events.Length; i++)
                {
                    UiEvent ev = events[i];
                    if (SoundEnabled && ev.SoundCue != (uint)SoundCueId.None)
                    {
                        PlayVanillaSound((SoundCueId)ev.SoundCue);
                    }

                    var kind = (UiEventKind)ev.Kind;
                    if (kind == UiEventKind.CloseRequested)
                    {
                        OnCloseRequested?.Invoke();
                        continue;
                    }

                    if (kind == UiEventKind.FocusInput && ev.ActionLen > 0)
                    {
                        FocusedInputId = Encoding.UTF8.GetString(pool.Slice((int)ev.ActionOffset, (int)ev.ActionLen));
                    }

                    if ((kind == UiEventKind.Click || kind == UiEventKind.ValueChanged) && ev.ActionLen > 0)
                    {
                        string actionName = Encoding.UTF8.GetString(pool.Slice((int)ev.ActionOffset, (int)ev.ActionLen));
                        if (_actionHandlers.TryGetValue(actionName, out var handler))
                        {
                            handler(ev);
                        }
                    }
                }
            }

            return isOverUi != 0;
        }

        public void Draw(SpriteBatch batch)
        {
            if (_tree == IntPtr.Zero)
                return;

            long start = System.Diagnostics.Stopwatch.GetTimestamp();
            LastCommandCount = _renderer.ExecuteRender(_tree, batch);
            long elapsed = System.Diagnostics.Stopwatch.GetTimestamp() - start;
            LastRenderMilliseconds = (double)elapsed * 1000.0 / System.Diagnostics.Stopwatch.Frequency;
        }

        private static void PlayVanillaSound(SoundCueId cue)
        {
            string? soundName = cue switch
            {
                SoundCueId.Shiny4 => "shiny4",
                SoundCueId.BigSelect => "bigSelect",
                SoundCueId.SmallSelect => "smallSelect",
                SoundCueId.Dwop => "dwop",
                SoundCueId.Coin => "coin",
                SoundCueId.Drumkit6 => "drumkit6",
                SoundCueId.Cancel => "cancel",
                SoundCueId.Trashcan => "trashcan",
                _ => null
            };
            if (soundName != null)
                Game1.playSound(soundName);
        }

        public void Dispose()
        {
            _hotReloadWatcher?.Dispose();
            _hotReloadWatcher = null;
            if (_tree != IntPtr.Zero)
            {
                NativeMethods.rusted_tree_destroy(_tree);
                _tree = IntPtr.Zero;
            }
        }
    }
}
