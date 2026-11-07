using System;
using StardewModdingAPI;
using StardewModdingAPI.Utilities;

namespace Rusted.Smapi
{
    /// <summary>
    /// Optional API interface for Spacechase0's Generic Mod Config Menu (GMCM).
    /// No hard dependency is required; if GMCM is not installed, RustedUI simply skips registration.
    /// </summary>
    public interface IGenericModConfigMenuApi
    {
        void Register(IManifest mod, Action reset, Action save, bool titleScreenOnly = false);

        void AddSectionTitle(IManifest mod, Func<string> text, Func<string>? tooltip = null);

        void AddParagraph(IManifest mod, Func<string> text);

        void AddBoolOption(
            IManifest mod,
            Func<bool> getValue,
            Action<bool> setValue,
            Func<string> name,
            Func<string>? tooltip = null,
            string? fieldId = null);

        void AddTextOption(
            IManifest mod,
            Func<string> getValue,
            Action<string> setValue,
            Func<string> name,
            Func<string>? tooltip = null,
            string[]? allowedValues = null,
            Func<string, string>? formatAllowedValue = null,
            string? fieldId = null);

        void AddKeybindList(
            IManifest mod,
            Func<KeybindList> getValue,
            Action<KeybindList> setValue,
            Func<string> name,
            Func<string>? tooltip = null,
            string? fieldId = null);
    }
}
