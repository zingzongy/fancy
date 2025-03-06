use bracket_lib::prelude::*;
use crate::{GameMode, FRAME_DURATION, OBSTACLE_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use super::{obstacle::Obstacle, player::Player};

pub struct State {
    player: Player,
    mode: GameMode,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}
impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(5, 5),
            mode: GameMode::Menu,
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    pub fn play(&mut self, ctx: &mut BTerm) {
        self.render_bg(ctx, NAVY);
        ctx.print(2, 1, "Press Space to Flap");
        ctx.print(2, 3, &format!("{}", self.score));
        if self.frame_time > FRAME_DURATION {
            self.player.gravity_move(ctx);
            self.player.render_player(ctx);
            self.obstacle.render_walls(ctx);
            self.obstacle.x -= OBSTACLE_SPEED;
            self.frame_time = 0.0;
        }
        self.frame_time += ctx.frame_time_ms;
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap(ctx);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(ctx, &self.player)  {
            self.mode = GameMode::Over;
        }
        if self.player.x as i32 == (self.obstacle.x as i32 + 1) {
            self.score += 1;
        }
        if self.obstacle.x < 0. {
            self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);
        }
    }
    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        self.render_bg(ctx, BLACK);
        ctx.print_centered(8, "Welcome to Flappy Clone!");
        ctx.print_centered(10, "(P) Press to Play");
        ctx.print_centered(12, "(Q) Press to Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(ctx),
                VirtualKeyCode::Q => ctx.quit() ,
                _ => (),
            }
        }
    }
    pub fn reset(&mut self, ctx: &mut BTerm) {
        self.player = Player::new(5, 5);
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
    pub fn game_over(&mut self, ctx: &mut BTerm) {
        self.clear_batch(ctx);
        self.render_bg(ctx, BLACK);
        ctx.print_centered(8, "You died!");
        ctx.print_centered(10, "(P) Press to Play");
        ctx.print_centered(12, "(Q) Press to Quit");
        ctx.print_centered(15, &format!("Score: {}", self.score));
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(ctx),
                VirtualKeyCode::Q => ctx.quit() ,
                _ => (),
            }
        }
    }
    pub fn clear_batch(&mut self, ctx: &mut BTerm) {
        let mut clear_batch = DrawBatch::new();
        clear_batch.target(1);
        clear_batch.cls();
        clear_batch.target(2);
        clear_batch.cls();
        clear_batch.submit(0).unwrap();
        render_draw_buffer(ctx).unwrap();
    }
    pub fn render_bg<COLOR>(&mut self, ctx: &mut BTerm, color: COLOR)
    where 
        COLOR: Into<RGBA>,
    {
        let mut draw_bg = DrawBatch::new();
        //target layer 0
        draw_bg.target(0);
        draw_bg.cls_color(color);
        draw_bg.submit(0).expect("problem submitting bg render");
        render_draw_buffer(ctx).expect("problem batching bg render");
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Over => self.game_over(ctx),
        }
    }
}
