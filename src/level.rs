use macroquad::prelude::*;

pub const LEVEL_WIDTH: f32 = 3. * 1024.;
pub const LEVEL_HEIGHT: f32 = 3. * 768.;

pub struct Level {
    pub ground: Rect,
    pub ceiling: Rect,
    pub left_wall: Rect,
    pub right_wall: Rect,
    pub platforms: Vec<Rect>,
}

impl Level {
    pub fn new() -> Self {
        let mut platforms = vec![];
        let screen_width = 1024.;
        let screen_height = 768.;

        for i in 0..3 { // columns
            for j in 0..3 { // rows
                let offset_x = i as f32 * screen_width;
                let offset_y = j as f32 * screen_height;

                // Define the platform layout relative to a screen's top-left corner
                let base_platforms = vec![
                    Rect::new(200., 168., 200., 20.),
                    Rect::new(500., 368., 200., 20.),
                    Rect::new(800., 568., 200., 20.),
                ];

                for platform in &base_platforms {
                    platforms.push(Rect::new(
                        offset_x + platform.x,
                        offset_y + platform.y,
                        platform.w,
                        platform.h,
                    ));
                }
            }
        }

        Self {
            ground: Rect::new(0., LEVEL_HEIGHT - 50., LEVEL_WIDTH, 50.),
            ceiling: Rect::new(0., 0., LEVEL_WIDTH, 50.),
            left_wall: Rect::new(0., 0., 50., LEVEL_HEIGHT),
            right_wall: Rect::new(LEVEL_WIDTH - 50., 0., 50., LEVEL_HEIGHT),
            platforms,
        }
    }

    pub fn draw(&self) {
        // Draw bounds
        draw_rectangle(self.ground.x, self.ground.y, self.ground.w, self.ground.h, YELLOW);
        draw_rectangle(self.ceiling.x, self.ceiling.y, self.ceiling.w, self.ceiling.h, YELLOW);
        draw_rectangle(self.left_wall.x, self.left_wall.y, self.left_wall.w, self.left_wall.h, YELLOW);
        draw_rectangle(self.right_wall.x, self.right_wall.y, self.right_wall.w, self.right_wall.h, YELLOW);

        // Draw platforms
        for platform in &self.platforms {
            draw_rectangle(platform.x, platform.y, platform.w, platform.h, GREEN);
        }
    }
}