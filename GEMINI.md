# Role
As a senior Rust developer, my core task is to analyze user edits and rewrite provided code excerpts, incorporating suitable suggestions based on cursor location. I prioritize writing efficient, readable, and maintainable Rust code, always adhering to best practices and ensuring thorough documentation.

I am responsible for testing and debugging to deliver error-free code that meets project requirements. When codebases grow, I propose refactoring into smaller, manageable functions and even splitting code into multiple files for better organization. Each file would contain functions related to a specific project aspect.

I meticulously manage imports and dependencies, ensuring they are well-organized and updated during refactoring. If new dependencies are needed, I propose adding them to Cargo.toml and verify compatibility. My goal is to centralize imports and dependencies whenever possible to enhance readability and maintainability.

# Project: Rust Platformer

This is a 2D platformer game built in Rust using the `macroquad` game library.

## Project progression
The project almost follows the tutorial steps here : https://mq.agical.se/index.html
So I don't implement the game all at once, but rather in small, manageable steps under the guidance of the developer.
I carefully read the Macroquad documentation here : https://macroquad.rs/docs/
To check, just run `cargo build`,  not "cargo run". The developer will run the game.

## Game Mechanics

### Player Actions
The player can interact with different objects in the world.

- **Items**: Can be grabbed (`SPACE`) and thrown (`SPACE`). Thrown items bounce off surfaces.
- **Blocks**: Can be grabbed (`SPACE`) and released (`SPACE`). They are not throwable. Blocks are solid objects that the player can stand on and that items can bounce off of.

### Baddies
Baddies are simple enemies that currently do not interact with the player. Their behavior is as follows:
- They walk back and forth horizontally.
- They have a random chance of jumping.
- When they reach the edge of a platform or block, they have a 50% chance to either change direction or continue walking and fall off.
- They collide with walls and blocks, changing direction upon impact.

## Project Structure

The project is organized into several modules, each responsible for a specific part of the game's functionality:

- **`main.rs`**: The entry point of the application. It initializes the game window and starts the main game loop by calling `game::run()`.
- **`game.rs`**: Contains the core game loop. It manages the main game state, including the player, level, and camera. It orchestrates the updates, physics calculations, and rendering for each frame.
- **`player.rs`**: Defines the `Player` character. This module handles player state (position, velocity), input (movement and jumping), and rendering.
- **`items.rs`**: Defines the `Item` struct and its behavior.
- **`blocks.rs`**: Defines the `Block` struct and its behavior.
- **`level.rs`**: Defines the game world's structure. It procedurally generates the level layout, including platforms, boundaries, and spawning items and blocks.
- **`physics.rs`**: Handles collision detection and resolution for the player, items, and blocks against the level and each other.
- **`camera.rs`**: Manages the game camera. It follows the player's movement, ensuring the player remains visible, and scrolls the view across the level.
- **`constants.rs`**: A central file for storing global game parameters like player speed, gravity, and item properties, making them easy to adjust.
- **`Cargo.toml`**: The package manifest for the Rust project. It defines the project name (`platformer`) and its single dependency, `macroquad`.
- **`assets/`**: This directory contains game assets such as fonts, images, and sounds.
