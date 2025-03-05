mod game;
use bracket_lib::prelude::*;
use game::player::Player;

const SCREEN_WIDTH: f32 = 80.;
const SCREEN_HEIGHT: f32 = 50.;
const FRAME_DURATION: f32 = 50.;

enum GameMode {
    Playing,
    Menu,
    Over,
}

struct State {
    player: Player,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 0),
            mode: GameMode::Playing,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        self.render_bg(ctx, RED);
        self.player.gravity_move(ctx);
        self.player.render_player(ctx);
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {

    }
    fn game_over(&mut self, ctx: &mut BTerm) {

    }
    fn render_bg<COLOR>(&mut self, ctx: &mut BTerm, color: COLOR)
    where 
        COLOR: Into<RGBA>,
    {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(0);
        draw_batch.cls_color(color);
        draw_batch.submit(0).unwrap();
        render_draw_buffer(ctx).unwrap();
        ()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Playing => self.play(ctx),
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Over => self.game_over(ctx),
        }
    }
}
fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Fancy Consoles")
        .with_vsync(false)
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}
