using System;
using System.Collections.Generic;
using System.Text;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Rusted.Smapi.Native;
using StardewValley;

namespace Rusted.Smapi.Rendering
{
    public sealed class RustedRenderer
    {
        private readonly Stack<Rectangle> _clipStack = new();
        private Rectangle _initialScissor;
        private static Texture2D? _lucideAtlasTexture;
        private static readonly RasterizerState ScissorRasterizer = new()
        {
            CullMode = CullMode.None,
            ScissorTestEnable = true
        };

        public static unsafe void SyncFontsToTree(IntPtr tree)
        {
            if (tree == IntPtr.Zero)
                return;

            SyncSingleFont(tree, (uint)VanillaFontId.Small, Game1.smallFont);
            SyncSingleFont(tree, (uint)VanillaFontId.Dialogue, Game1.dialogueFont);
            SyncSingleFont(tree, (uint)VanillaFontId.Tiny, Game1.tinyFont);
        }

        private static unsafe void SyncSingleFont(IntPtr tree, uint fontId, SpriteFont? font)
        {
            if (font == null)
                return;

            Dictionary<char, SpriteFont.Glyph> glyphs = font.GetGlyphs();
            if (glyphs.Count == 0)
                return;

            NativeGlyphMetric[] buffer = new NativeGlyphMetric[glyphs.Count];
            int idx = 0;
            foreach (var kvp in glyphs)
            {
                SpriteFont.Glyph g = kvp.Value;
                buffer[idx++] = new NativeGlyphMetric
                {
                    Character = kvp.Key,
                    Advance = g.Width,
                    LeftBearing = g.LeftSideBearing,
                    RightBearing = g.RightSideBearing
                };
            }

            fixed (NativeGlyphMetric* ptr = buffer)
            {
                NativeMethods.rusted_tree_sync_font(
                    tree,
                    fontId,
                    font.LineSpacing,
                    font.Spacing,
                    ptr,
                    (uint)buffer.Length);
            }
        }

        public unsafe int ExecuteRender(IntPtr tree, SpriteBatch batch)
        {
            if (tree == IntPtr.Zero)
                return 0;

            float viewportW = Game1.uiViewport.Width;
            float viewportH = Game1.uiViewport.Height;

            RenderSliceView view;
            uint count = NativeMethods.rusted_tree_render(tree, viewportW, viewportH, &view);
            if (count == 0 || view.CommandsPtr == null)
                return 0;

            ReadOnlySpan<RenderCommand> commands = new(view.CommandsPtr, (int)view.CommandsLen);
            ReadOnlySpan<byte> textPool = view.TextPoolPtr != null
                ? new ReadOnlySpan<byte>(view.TextPoolPtr, (int)view.TextPoolLen)
                : ReadOnlySpan<byte>.Empty;

            GraphicsDevice device = Game1.graphics.GraphicsDevice;
            _clipStack.Clear();
            _initialScissor = device.ScissorRectangle;

            for (int i = 0; i < commands.Length; i++)
            {
                ref readonly RenderCommand cmd = ref commands[i];
                Color color = new(cmd.Color.R, cmd.Color.G, cmd.Color.B, cmd.Color.A);

                switch ((RenderCommandKind)cmd.Kind)
                {
                    case RenderCommandKind.DrawNineSlice:
                    {
                        Texture2D tex = ResolveTexture((TextureId)cmd.TextureId);
                        NineSliceRenderer.Draw(batch, tex, in cmd.DestRect, in cmd.SourceRect, in cmd.Insets, color, cmd.Scale);
                        break;
                    }

                    case RenderCommandKind.DrawSprite:
                    {
                        Texture2D tex = ResolveTexture((TextureId)cmd.TextureId);
                        Rectangle dest = new((int)cmd.DestRect.X, (int)cmd.DestRect.Y, (int)cmd.DestRect.Width, (int)cmd.DestRect.Height);
                        Rectangle src = new(cmd.SourceRect.X, cmd.SourceRect.Y, cmd.SourceRect.Width, cmd.SourceRect.Height);
                        batch.Draw(tex, dest, src, color);
                        break;
                    }

                    case RenderCommandKind.DrawRect:
                    {
                        Rectangle dest = new((int)cmd.DestRect.X, (int)cmd.DestRect.Y, (int)cmd.DestRect.Width, (int)cmd.DestRect.Height);
                        batch.Draw(Game1.staminaRect, dest, color);
                        break;
                    }

                    case RenderCommandKind.DrawText:
                    {
                        if (cmd.TextLen == 0)
                            break;

                        SpriteFont font = ResolveFont((VanillaFontId)cmd.FontId);
                        ReadOnlySpan<byte> slice = textPool.Slice((int)cmd.TextOffset, (int)cmd.TextLen);
                        string text = Encoding.UTF8.GetString(slice);
                        Vector2 pos = new(cmd.DestRect.X, cmd.DestRect.Y);

                        if (cmd.ShadowColor.A > 0)
                        {
                            Color shadow = new(cmd.ShadowColor.R, cmd.ShadowColor.G, cmd.ShadowColor.B, cmd.ShadowColor.A);
                            Vector2 shadowPos = new(pos.X + cmd.ShadowOffsetX, pos.Y + cmd.ShadowOffsetY);
                            batch.DrawString(font, text, shadowPos, shadow, 0f, Vector2.Zero, cmd.Scale, SpriteEffects.None, 0.99f);
                        }

                        batch.DrawString(font, text, pos, color, 0f, Vector2.Zero, cmd.Scale, SpriteEffects.None, 1f);
                        break;
                    }

                    case RenderCommandKind.PushClipRect:
                    {
                        PushClip(batch, in cmd.DestRect);
                        break;
                    }

                    case RenderCommandKind.PopClipRect:
                    {
                        PopClip(batch);
                        break;
                    }
                }
            }

            if (_clipStack.Count > 0)
            {
                _clipStack.Clear();
                batch.End();
                device.ScissorRectangle = _initialScissor;
                batch.Begin(
                    SpriteSortMode.Deferred,
                    BlendState.AlphaBlend,
                    SamplerState.PointClamp,
                    null,
                    RasterizerState.CullNone);
            }

            return commands.Length;
        }

        private void PushClip(SpriteBatch batch, in RectF uiRect)
        {
            GraphicsDevice device = Game1.graphics.GraphicsDevice;
            batch.End();

            if (_clipStack.Count == 0)
            {
                _initialScissor = device.ScissorRectangle;
            }

            float scaleX = (float)device.Viewport.Width / Math.Max(1f, Game1.uiViewport.Width);
            float scaleY = (float)device.Viewport.Height / Math.Max(1f, Game1.uiViewport.Height);

            int sx = (int)Math.Floor(uiRect.X * scaleX);
            int sy = (int)Math.Floor(uiRect.Y * scaleY);
            int sw = (int)Math.Ceiling(uiRect.Width * scaleX);
            int sh = (int)Math.Ceiling(uiRect.Height * scaleY);

            Rectangle hwRect = new(sx, sy, Math.Max(1, sw), Math.Max(1, sh));
            hwRect = Rectangle.Intersect(device.Viewport.Bounds, hwRect);

            if (_clipStack.Count > 0)
                hwRect = Rectangle.Intersect(_clipStack.Peek(), hwRect);

            _clipStack.Push(hwRect);
            device.ScissorRectangle = hwRect;

            batch.Begin(
                SpriteSortMode.Deferred,
                BlendState.AlphaBlend,
                SamplerState.PointClamp,
                null,
                ScissorRasterizer);
        }

        private void PopClip(SpriteBatch batch)
        {
            GraphicsDevice device = Game1.graphics.GraphicsDevice;
            batch.End();

            if (_clipStack.Count > 0)
                _clipStack.Pop();

            if (_clipStack.Count > 0)
            {
                device.ScissorRectangle = _clipStack.Peek();
                batch.Begin(
                    SpriteSortMode.Deferred,
                    BlendState.AlphaBlend,
                    SamplerState.PointClamp,
                    null,
                    ScissorRasterizer);
            }
            else
            {
                device.ScissorRectangle = _initialScissor;
                batch.Begin(
                    SpriteSortMode.Deferred,
                    BlendState.AlphaBlend,
                    SamplerState.PointClamp,
                    null,
                    RasterizerState.CullNone);
            }
        }

        private static Texture2D ResolveTexture(TextureId id)
        {
            return id switch
            {
                TextureId.Cursors => Game1.mouseCursors,
                TextureId.Cursors2 => Game1.mouseCursors2,
                TextureId.MenuTiles => Game1.menuTexture,
                TextureId.MenuTilesUncolored => Game1.uncoloredMenuTexture,
                TextureId.Objects => Game1.objectSpriteSheet,
                TextureId.LucideAtlas => GetOrCreateLucideAtlas(),
                _ => Game1.mouseCursors
            };
        }

        private static Texture2D GetOrCreateLucideAtlas()
        {
            if (_lucideAtlasTexture != null && !_lucideAtlasTexture.IsDisposed)
                return _lucideAtlasTexture;

            GraphicsDevice device = Game1.graphics.GraphicsDevice;
            const int width = 256;
            const int height = 16;
            Color[] pixels = new Color[width * height];

            string[][] glyphs =
            {
                // 0: sparkles
                new[] {
                    ".......##.......",
                    ".......##.......",
                    "......####......",
                    "....########....",
                    "..############..",
                    "....########....",
                    "......####..##..",
                    ".......##..####.",
                    "..##........##..",
                    ".####...........",
                    "..##.....##.....",
                    "........####....",
                    ".......######...",
                    "........####....",
                    ".........##.....",
                    "................"
                },
                // 1: cpu
                new[] {
                    "....##.##.##....",
                    "....##.##.##....",
                    "..############..",
                    "..#..........#..",
                    "###..######..###",
                    "..#..#....#..#..",
                    "###..#.##.#..###",
                    "..#..#.##.#..#..",
                    "###..#....#..###",
                    "..#..######..#..",
                    "###..........###",
                    "..############..",
                    "....##.##.##....",
                    "....##.##.##....",
                    "................",
                    "................"
                },
                // 2: palette
                new[] {
                    "....########....",
                    "..############..",
                    ".###..####..###.",
                    "###...####...###",
                    "##.##......##.##",
                    "##.##......##.##",
                    "##............##",
                    "###..##.......##",
                    "###..##...######",
                    ".###.....###....",
                    "..#########.....",
                    "....######......",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 3: code
                new[] {
                    ".......##.......",
                    "......##........",
                    "....##..##..##..",
                    "...##...##...##.",
                    "..##...##.....##",
                    ".##....##......#",
                    "..##...##.....##",
                    "...##..##....##.",
                    "....##.##...##..",
                    "......##........",
                    ".....##.........",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 4: zap
                new[] {
                    "........####....",
                    ".......####.....",
                    "......####......",
                    ".....####.......",
                    "....#########...",
                    "...##########...",
                    ".......####.....",
                    "......####......",
                    ".....####.......",
                    "....####........",
                    "...###..........",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 5: layers
                new[] {
                    ".......##.......",
                    ".....######.....",
                    "...##########...",
                    ".##############.",
                    "...##########...",
                    ".....######.....",
                    ".##....##....##.",
                    "...##########...",
                    ".....######.....",
                    ".##....##....##.",
                    "...##########...",
                    ".....######.....",
                    ".......##.......",
                    "................",
                    "................",
                    "................"
                },
                // 6: sliders
                new[] {
                    "...##....##...##",
                    "...##....##...##",
                    ".######..##...##",
                    ".######..##...##",
                    "...##..######.##",
                    "...##..######.##",
                    "...##....##.####",
                    "...##....##.####",
                    "...##....##...##",
                    "...##....##...##",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 7: package
                new[] {
                    "......####......",
                    "....########....",
                    "..############..",
                    ".####..##..####.",
                    ".##....##....##.",
                    ".##....##....##.",
                    ".##....##....##.",
                    ".####..##..####.",
                    "..############..",
                    "....########....",
                    "......####......",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 8: heart
                new[] {
                    "..####....####..",
                    ".######..######.",
                    "################",
                    "################",
                    "################",
                    ".##############.",
                    "..############..",
                    "...##########...",
                    "....########....",
                    ".....######.....",
                    "......####......",
                    ".......##.......",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 9: star
                new[] {
                    ".......##.......",
                    "......####......",
                    "......####......",
                    ".##############.",
                    "..############..",
                    "...##########...",
                    "....########....",
                    "...####..####...",
                    "..####....####..",
                    ".###........###.",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 10: shield
                new[] {
                    "......####......",
                    "...##########...",
                    "..############..",
                    "..##........##..",
                    "..##........##..",
                    "..##........##..",
                    "...##......##...",
                    "....##....##....",
                    ".....######.....",
                    "......####......",
                    ".......##.......",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 11: sword
                new[] {
                    "............###.",
                    "...........####.",
                    "..........####..",
                    ".........####...",
                    "........####....",
                    "..##...####.....",
                    "...#######......",
                    "....#####.......",
                    "...#####........",
                    "..###..##.......",
                    ".###............",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 12: flame
                new[] {
                    ".......##.......",
                    "......####......",
                    ".....######.....",
                    "....########....",
                    "...####..####...",
                    "..####....####..",
                    "..###..##..###..",
                    "..####....####..",
                    "...##########...",
                    "....########....",
                    "......####......",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 13: terminal
                new[] {
                    ".##############.",
                    ".##..........##.",
                    ".##.##.......##.",
                    ".##..##......##.",
                    ".##...##.....##.",
                    ".##..##......##.",
                    ".##.##..####.##.",
                    ".##..........##.",
                    ".##############.",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 14: check
                new[] {
                    ".............##.",
                    "............###.",
                    "...........###..",
                    "..........###...",
                    "..##.....###....",
                    ".####...###.....",
                    "..####.###......",
                    "...######.......",
                    "....####........",
                    ".....##.........",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                },
                // 15: settings
                new[] {
                    "......####......",
                    "..##..####..##..",
                    ".##############.",
                    "...###....###...",
                    "..###......###..",
                    "..###......###..",
                    "...###....###...",
                    ".##############.",
                    "..##..####..##..",
                    "......####......",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................",
                    "................"
                }
            };

            for (int iconIdx = 0; iconIdx < glyphs.Length; iconIdx++)
            {
                string[] rows = glyphs[iconIdx];
                int baseX = iconIdx * 16;
                for (int y = 0; y < 16 && y < rows.Length; y++)
                {
                    string row = rows[y];
                    for (int x = 0; x < 16 && x < row.Length; x++)
                    {
                        if (row[x] == '#')
                        {
                            pixels[y * width + baseX + x] = Color.White;
                        }
                    }
                }
            }

            _lucideAtlasTexture = new Texture2D(device, width, height);
            _lucideAtlasTexture.SetData(pixels);
            return _lucideAtlasTexture;
        }

        private static SpriteFont ResolveFont(VanillaFontId id)
        {
            return id switch
            {
                VanillaFontId.Dialogue => Game1.dialogueFont,
                VanillaFontId.Tiny => Game1.tinyFont ?? Game1.smallFont,
                _ => Game1.smallFont
            };
        }
    }
}
