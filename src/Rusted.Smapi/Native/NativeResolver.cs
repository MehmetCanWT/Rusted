using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using StardewModdingAPI;

namespace Rusted.Smapi.Native
{
    public static class NativeResolver
    {
        private static string _modDirectory = string.Empty;
        private static IntPtr _loadedHandle = IntPtr.Zero;
        private static bool _initialized;

        public static void Initialize(IModHelper helper, IMonitor monitor)
        {
            if (_initialized)
                return;

            _modDirectory = helper.DirectoryPath;
            NativeLibrary.SetDllImportResolver(typeof(NativeResolver).Assembly, (libName, asm, path) =>
                ResolveDllImport(libName, monitor));
            _initialized = true;
        }

        private static IntPtr ResolveDllImport(string libraryName, IMonitor monitor)
        {
            if (libraryName != NativeMethods.LibName)
                return IntPtr.Zero;

            if (_loadedHandle != IntPtr.Zero)
                return _loadedHandle;

            string relativePath = GetPlatformBinaryRelativePath();
            string fullPath = Path.Combine(_modDirectory, relativePath);

            if (!File.Exists(fullPath))
            {
                monitor.Log($"[RustedUI] Native binary not found at: {fullPath} (OS: {RuntimeInformation.OSDescription})", LogLevel.Error);
                throw new FileNotFoundException($"[RustedUI] Native binary missing: {fullPath}");
            }

            if (NativeLibrary.TryLoad(fullPath, out _loadedHandle))
            {
                monitor.Log($"[RustedUI] Loaded native Rust engine ({relativePath}) successfully.", LogLevel.Info);
                return _loadedHandle;
            }

            throw new DllNotFoundException($"[RustedUI] Failed to load native library: {fullPath}");
        }

        private static string GetPlatformBinaryRelativePath()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                return Path.Combine("runtimes", "win-x64", "native", "rusted_bridge.dll");
            }

            if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            {
                return Path.Combine("runtimes", "linux-x64", "native", "librusted_bridge.so");
            }

            if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                return Path.Combine("runtimes", "osx", "native", "librusted_bridge.dylib");
            }

            throw new PlatformNotSupportedException($"[RustedUI] Unsupported OS: {RuntimeInformation.OSDescription}");
        }
    }
}
