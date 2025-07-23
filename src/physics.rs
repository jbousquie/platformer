use macroquad::prelude::*;
use crate::player::Player;
use crate::level::Level;

pub fn resolve_collisions(player: &mut Player, level: &Level) {
    player.on_ground = false;

    // Player vs. Level bounds
    if player.rect.overlaps(&level.left_wall) {
        player.rect.x = level.left_wall.right();
    }
    if player.rect.overlaps(&level.right_wall) {
        player.rect.x = level.right_wall.left() - player.rect.w;
    }
    if player.rect.overlaps(&level.ceiling) {
        player.rect.y = level.ceiling.bottom();
        player.velocity.y = 0.;
    }
    if player.rect.overlaps(&level.ground) {
        if player.velocity.y > 0. {
            player.rect.y = level.ground.y - player.rect.h;
            player.velocity.y = 0.;
            player.on_ground = true;
        }
    }
}