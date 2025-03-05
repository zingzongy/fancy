use bracket_lib::prelude::*;

const SCREEN_WIDTH: f32 = 80.;
const SCREEN_HEIGHT: f32 = 50.;
const FRAME_DURATION: f32 = 50.;

enum GameMode {
    Playing,
    Menu,
    Over,
}

struct Player {
    x: f32,
    y: f32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x: x as f32,
            y: y as f32,
            velocity: 0.0,
        }
    }
    fn gravity_move(&mut self, ctx: &mut BTerm) {
        if self.velocity < 2.0 {
            self.velocity += 0.01;
        }
        self.y += self.velocity;
    }
    fn render_player(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);
        draw_batch.cls();
        draw_batch.set_fancy(
            PointF::new(self.x, self.y) ,
            0, 
            Degrees::new(0.),
            PointF::new(1., 1.),
            ColorPair::new(YELLOW, BLACK),
            to_cp437('@'),
            );
        //submission
        draw_batch.submit(1).unwrap();
        render_draw_buffer(ctx).unwrap();
    }
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
