use macroquad::prelude::*;
use crate::player::Player;
use crate::level::{LEVEL_WIDTH, LEVEL_HEIGHT};

pub struct Camera {
    pub rect: Rect,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0., 0., screen_width(), screen_height()),
        }
    }

    pub fn update(&mut self, player: &Player) {
        let screen_quarter_w = screen_width() / 4.;
        let screen_quarter_h = screen_height() / 4.;

        // Scroll left
        if player.rect.x < self.rect.x + screen_quarter_w {
            self.rect.x = player.rect.x - screen_quarter_w;
        }
        // Scroll right
        if player.rect.right() > self.rect.right() - screen_quarter_w {
            self.rect.x = player.rect.right() - self.rect.w + screen_quarter_w;
        }
        // Scroll up
        if player.rect.y < self.rect.y + screen_quarter_h {
            self.rect.y = player.rect.y - screen_quarter_h;
        }
        // Scroll down
        if player.rect.bottom() > self.rect.bottom() - screen_quarter_h {
            self.rect.y = player.rect.bottom() - self.rect.h + screen_quarter_h;
        }

        // Clamp camera to level bounds
        self.rect.x = self.rect.x.max(0.).min(LEVEL_WIDTH - self.rect.w);
        self.rect.y = self.rect.y.max(0.).min(LEVEL_HEIGHT - self.rect.h);
    }
}