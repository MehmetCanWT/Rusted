using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Rusted.Smapi.Native;

namespace Rusted.Smapi.Rendering
{
    /// <summary>
    /// Zero-allocation 9-Slice renderer matching Stardew Valley's IClickableMenu.drawTextureBox.
    /// </summary>
    public static class NineSliceRenderer
    {
        public static void Draw(
            SpriteBatch batch,
            Texture2D texture,
            in RectF dest,
            in RectI src,
            in BorderInsets insets,
            Color tint,
            float scale)
        {
            int srcTop = (int)insets.Top;
            int srcRight = (int)insets.Right;
            int srcBottom = (int)insets.Bottom;
            int srcLeft = (int)insets.Left;

            int dstTop = (int)(insets.Top * scale);
            int dstRight = (int)(insets.Right * scale);
            int dstBottom = (int)(insets.Bottom * scale);
            int dstLeft = (int)(insets.Left * scale);

            int x = (int)dest.X;
            int y = (int)dest.Y;
            int w = (int)dest.Width;
            int h = (int)dest.Height;

            int centerSrcW = src.Width - srcLeft - srcRight;
            int centerSrcH = src.Height - srcTop - srcBottom;
            int centerDstW = w - dstLeft - dstRight;
            int centerDstH = h - dstTop - dstBottom;

            // 1. Center Fill
            if (centerDstW > 0 && centerDstH > 0 && centerSrcW > 0 && centerSrcH > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + dstLeft, y + dstTop, centerDstW, centerDstH),
                    new Rectangle(src.X + srcLeft, src.Y + srcTop, centerSrcW, centerSrcH),
                    tint);
            }

            // 2. Four Stretched Edges
            if (centerDstW > 0 && srcTop > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + dstLeft, y, centerDstW, dstTop),
                    new Rectangle(src.X + srcLeft, src.Y, centerSrcW, srcTop),
                    tint);
            }
            if (centerDstW > 0 && srcBottom > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + dstLeft, y + h - dstBottom, centerDstW, dstBottom),
                    new Rectangle(src.X + srcLeft, src.Y + src.Height - srcBottom, centerSrcW, srcBottom),
                    tint);
            }
            if (centerDstH > 0 && srcLeft > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x, y + dstTop, dstLeft, centerDstH),
                    new Rectangle(src.X, src.Y + srcTop, srcLeft, centerSrcH),
                    tint);
            }
            if (centerDstH > 0 && srcRight > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + w - dstRight, y + dstTop, dstRight, centerDstH),
                    new Rectangle(src.X + src.Width - srcRight, src.Y + srcTop, srcRight, centerSrcH),
                    tint);
            }

            // 3. Four Fixed Corners
            if (dstLeft > 0 && dstTop > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x, y, dstLeft, dstTop),
                    new Rectangle(src.X, src.Y, srcLeft, srcTop),
                    tint);
            }
            if (dstRight > 0 && dstTop > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + w - dstRight, y, dstRight, dstTop),
                    new Rectangle(src.X + src.Width - srcRight, src.Y, srcRight, srcTop),
                    tint);
            }
            if (dstLeft > 0 && dstBottom > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x, y + h - dstBottom, dstLeft, dstBottom),
                    new Rectangle(src.X, src.Y + src.Height - srcBottom, srcLeft, srcBottom),
                    tint);
            }
            if (dstRight > 0 && dstBottom > 0)
            {
                batch.Draw(
                    texture,
                    new Rectangle(x + w - dstRight, y + h - dstBottom, dstRight, dstBottom),
                    new Rectangle(src.X + src.Width - srcRight, src.Y + src.Height - srcBottom, srcRight, srcBottom),
                    tint);
            }
        }
    }
}
