using StardewModdingAPI;
using StardewModdingAPI.Utilities;

namespace Rusted.Smapi
{
    public sealed class ModConfig
    {
        public bool ShowHud { get; set; } = false;
        public bool EnableUiSounds { get; set; } = true;
        public KeybindList OpenMenuKey { get; set; } = KeybindList.Parse("F8");
        public KeybindList ToggleHudKey { get; set; } = KeybindList.Parse("F7");
        public string DefaultTheme { get; set; } = "PelicanClassic";
    }
}
