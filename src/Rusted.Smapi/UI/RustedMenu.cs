using System;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using StardewValley;
using StardewValley.Menus;

namespace Rusted.Smapi.UI
{
    /// <summary>
    /// Seamless Stardew Valley IClickableMenu powered 100% by a RustedView.
    /// </summary>
    public sealed class RustedMenu : IClickableMenu
    {
        private readonly RustedView _view;
        private readonly Action<Keys>? _onKeyPress;

        public RustedView View => _view;

        public RustedMenu(RustedView view, Action<Keys>? onKeyPress = null)
            : base(0, 0, Game1.uiViewport.Width, Game1.uiViewport.Height, showUpperRightCloseButton: false)
        {
            _view = view;
            _onKeyPress = onKeyPress;
            _view.OnCloseRequested += ExitMenu;
        }

        private void ExitMenu()
        {
            if (Game1.activeClickableMenu == this)
                exitThisMenu(playSound: false);
        }

        public override void receiveKeyPress(Keys key)
        {
            if (key == Keys.Escape)
            {
                Game1.playSound("drumkit6");
                exitThisMenu(playSound: false);
                return;
            }

            if (_onKeyPress != null)
            {
                _onKeyPress(key);
                return;
            }

            if (Game1.options.doesInputListContain(Game1.options.menuButton, key))
            {
                Game1.playSound("drumkit6");
                exitThisMenu(playSound: false);
                return;
            }
            base.receiveKeyPress(key);
        }

        public override void draw(SpriteBatch b)
        {
            // Dim background like vanilla menus
            if (!Game1.options.showClearBackgrounds)
            {
                b.Draw(Game1.fadeToBlackRect, Game1.graphics.GraphicsDevice.Viewport.Bounds, Microsoft.Xna.Framework.Color.Black * 0.55f);
            }

            _view.Draw(b);
            drawMouse(b);
        }
    }
}
