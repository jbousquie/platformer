# PLATFORMER GAME

A attempt to create a platformer game using Rust and [macroquad](https://github.com/not-fl3/macroquad) using mainly Gemini-CLI.

[Macroquad Site](https://macroquad.rs/)

## How to Play

-   **Move Left/Right**: Use the Left and Right arrow keys.
-   **Jump**: Press the Up arrow key.
-   **Grab/Throw Items**: Press the `SPACE` key to grab a nearby item. Press `SPACE` again to throw it.
-   **Grab/Release Blocks**: Press the `SPACE` key to grab a nearby block, or to release one you are holding. Blocks cannot be thrown.

## GUI

A simple GUI is displayed at the top of the screen, showing:
- **Keys**: The number of keys collected out of the total.
- **Score**: The player's score (placeholder).
- **Lives**: The number of remaining lives (placeholder).


## Game Entities

### Baddies

Baddies are simple enemies that roam the level. They walk back and forth, randomly jump, and can fall off platforms. Upon colliding with a block, they have a small chance to pick it up and carry it for a while. They do not interact with the player.

### Keys

Keys are collectible items. The goal of the game is to collect all of them.


## License & Credits

### LICENSE for all assets:

Creative Commons Zero (CC0)

### CREDIT:

**SPRITES by analogStudios_:**
- knight (https://analogstudios.itch.io/camelot)
- slime (https://analogstudios.itch.io/dungeonsprites)
- platforms and coin (https://analogstudios.itch.io/four-seasons-platformer-sprites)

**SPRITES by RottingPixels:**
- world_tileset and fruit (https://rottingpixels.itch.io/four-seasons-platformer-tileset-16x16free)

**SOUNDS by Brackeys, Asbjørn Thirslund**

**MUSIC by Brackeys, Sofia Thirslund**

**FONTS by Jayvee Enaguas - HarvettFox96 - (https://www.dafont.com/pixel-operator.font?l[]=10&l[]=1)**