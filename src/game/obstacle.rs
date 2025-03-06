use bracket_lib::prelude::*;

use crate::SCREEN_HEIGHT;

use super::player::Player;

pub struct Obstacle {
    pub x: f32,
    gap_y: f32,
    size: i32,
}
impl Obstacle {
    pub fn new(x: f32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10., 40.),
            size: i32::max(2, 20 - score),
        }
    }
    pub fn render_walls(&mut self, ctx: &mut BTerm) {
        let half_size = self.size / 2;
        //preparing to draw
        let mut draw_obstacle = DrawBatch::new();
        draw_obstacle.target(2);
        draw_obstacle.cls();
        //draw top half of wall
        for y in 0..(self.gap_y as i32 - half_size) {
            draw_obstacle.set_fancy(
                PointF { x: self.x, y: y as f32 },
                1,
                Degrees::new(0.0),
                PointF { x: 1.0, y: 1.0 },
                ColorPair::new(RED, BLACK),
                to_cp437('|'),
                );
        }
        //draw bottom half of wall
        for y in (self.gap_y as i32 + half_size)..(SCREEN_HEIGHT as i32 + 1) {
            draw_obstacle.set_fancy(
                PointF { x: self.x, y: y as f32 },
                1,
                Degrees::new(0.0),
                PointF { x: 1.0, y: 1.0 },
                ColorPair::new(RED, BLACK),
                to_cp437('|'),
                );
        }
        draw_obstacle.submit(0).expect("problem submitting obstacle batch");
        render_draw_buffer(ctx).expect("problem batching and emptying obstacle render");
    }
    pub fn hit_obstacle(&mut self, ctx: &mut BTerm, player: &Player) -> bool {
        let half_size = self.size as f32 / 2.;
        let does_x_match = player.x as i32 == self.x as i32;
        let above_gap = player.y < self.gap_y - half_size;
        let below_gap = player.y > self.gap_y + half_size;
        does_x_match && (above_gap || below_gap)
    }
}
