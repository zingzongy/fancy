use bracket_lib::prelude::*;
use crate::GameMode;
use super::player::Player;

pub struct State {
    player: Player,
    mode: GameMode,
}
impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(5, 5),
            mode: GameMode::Menu,
        }
    }
    pub fn play(&mut self, ctx: &mut BTerm) {
    }
    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        self.render_bg(ctx, YELLOW);
    }
    pub fn game_over(&mut self, ctx: &mut BTerm) {

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
