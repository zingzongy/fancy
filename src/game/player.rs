use bracket_lib::prelude::*;

use crate::SCREEN_HEIGHT;

pub struct Player {
    pub x: f32,
    pub y: f32,
    velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x: x as f32,
            y: y as f32,
            velocity: 0.0,
        }
    }
    pub fn gravity_move(&mut self, ctx: &mut BTerm) {
        if self.velocity < 2.0 {
            self.velocity += 0.05;
        }
        self.y += self.velocity;
        self.y = self.y.clamp(1.0, SCREEN_HEIGHT + 5.);
    }
    pub fn flap(&mut self, ctx: &mut BTerm) {
        self.velocity -= 0.5;
        self.velocity = self.velocity.clamp(-1., 2.);
    }
    pub fn render_player(&mut self, ctx: &mut BTerm) {
        let mut draw_player = DrawBatch::new();
        //targeting layer 1
        draw_player.target(1);
        draw_player.cls();
        draw_player.set_fancy(
            PointF { x: 5.0, y: self.y }, 
            1,
            Degrees::new(0.0), 
            PointF { x: 1.0, y: 1.0 },
            ColorPair::new(YELLOW, BLACK),
            to_cp437('@'),
            );
        draw_player.submit(0).expect("render player submission failed");
        render_draw_buffer(ctx).expect("render draw buffer failed");
    }
}
