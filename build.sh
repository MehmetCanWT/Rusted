#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"
export PATH="$HOME/.dotnet:$HOME/.cargo/bin:$PATH"

echo "==> [0/4] Compiling React/TSX + Shadcn/UI + Lucide components (ui-tsx)..."
node ui-tsx/scripts/build-tsx.mjs

echo "==> [1/4] Running Rust workspace unit tests..."
cargo test --workspace

echo "==> [2/4] Building Linux native library (librusted_bridge.so)..."
cargo build --release
mkdir -p src/Rusted.Smapi/runtimes/linux-x64/native
cp target/release/librusted_bridge.so src/Rusted.Smapi/runtimes/linux-x64/native/

echo "==> [3/4] Building Windows & Proton native library (rusted_bridge.dll with +crt-static)..."
cargo xwin build --release --target x86_64-pc-windows-msvc
mkdir -p src/Rusted.Smapi/runtimes/win-x64/native
cp target/x86_64-pc-windows-msvc/release/rusted_bridge.dll src/Rusted.Smapi/runtimes/win-x64/native/

echo "==> [4/4] Building & deploying C# SMAPI mod (Rusted.Smapi)..."
dotnet build src/Rusted.Smapi/Rusted.Smapi.csproj -c Release

GAME_MODS_DIR="$HOME/.local/share/Steam/steamapps/common/Stardew Valley/Mods"
if [ -d "$GAME_MODS_DIR" ]; then
    echo "==> Deploying RustedUI to $GAME_MODS_DIR/RustedUI..."
    rm -rf "$GAME_MODS_DIR/RustedUI" "$GAME_MODS_DIR/Rusted.Smapi"
    unzip -q -o "src/Rusted.Smapi/bin/Release/net6.0/RustedUI 0.1.0.zip" -d "$GAME_MODS_DIR"
    chmod -R u+rwX "$GAME_MODS_DIR/RustedUI"
    cp "src/Rusted.Smapi/bin/Release/net6.0/RustedUI 0.1.0.zip" "RustedUI-0.1.0.zip"
fi

echo "✅ RustedUI build and deployment complete!"
