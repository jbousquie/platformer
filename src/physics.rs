//! Physics Module
//!
//! This module handles collision detection and resolution.

use crate::items::Item;
use crate::level::Level;
use crate::player::Player;
use macroquad::prelude::{get_frame_time, Rect};

/// Resolves collisions between the player and the level, including boundaries and platforms.
pub fn resolve_player_collisions(player: &mut Player, level: &Level) {
    player.on_ground = false;
    let player_rect = player.rect();

    // Player vs. Level bounds
    if player_rect.overlaps(&level.left_wall) {
        player.position.x = level.left_wall.right();
    }
    if player_rect.overlaps(&level.right_wall) {
        player.position.x = level.right_wall.left() - player.size.x;
    }
    if player_rect.overlaps(&level.ceiling) {
        player.position.y = level.ceiling.bottom();
        player.velocity.y = 0.;
    }
    if player_rect.overlaps(&level.ground) {
        if player.velocity.y > 0. {
            player.position.y = level.ground.y - player.size.y;
            player.velocity.y = 0.;
            player.on_ground = true;
        }
    }

    // Player vs. Platforms
    if player.velocity.y > 0. {
        for platform in &level.platforms {
            let player_rect = player.rect();
            if player_rect.overlaps(platform) {
                // Check if the player's bottom edge was above the platform's top edge in the previous frame
                let previous_player_bottom =
                    player.position.y + player.size.y - player.velocity.y * get_frame_time();
                if previous_player_bottom <= platform.y {
                    player.position.y = platform.y - player.size.y;
                    player.velocity.y = 0.;
                    player.on_ground = true;
                }
            }
        }
    }
}

/// Resolves collisions for a single item with the level.
pub fn resolve_item_collisions(item: &mut Item, ground: &Rect, platforms: &[Rect]) {
    item.on_ground = false;
    let item_rect = item.rect();

    // Item vs. Level ground
    if item_rect.overlaps(ground) {
        if item.velocity.y > 0. {
            item.position.y = ground.y - item.size.y;
            item.velocity.y = 0.;
            item.on_ground = true;
        }
    }

    // Item vs. Platforms
    if item.velocity.y > 0. {
        for platform in platforms {
            let item_rect = item.rect();
            if item_rect.overlaps(platform) {
                let previous_item_bottom =
                    item.position.y + item.size.y - item.velocity.y * get_frame_time();
                if previous_item_bottom <= platform.y {
                    item.position.y = platform.y - item.size.y;
                    item.velocity.y = 0.;
                    item.on_ground = true;
                }
            }
        }
    }
}

