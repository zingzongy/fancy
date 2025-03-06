mod game;

use bracket_lib::prelude::*;
use game::state::State;
const OBSTACLE_SPEED: f32 = 0.8; //lower is slower 
const SCREEN_WIDTH: f32 = 80.;
const SCREEN_HEIGHT: f32 = 50.;
const FRAME_DURATION: f32 = 15.;

enum GameMode {
    Playing,
    Menu,
    Over,
}
fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")//layer 1
        .with_fancy_console(80, 50, "terminal8x8.png")//layer 2
        .with_title("Bracket Terminal - Fancy Consoles")
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}
