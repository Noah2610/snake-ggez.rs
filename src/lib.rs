extern crate ggez;
extern crate rand;

use ggez::{
  GameResult,
  Context,
  graphics,
  event
};

mod settings;
mod color;
mod geo;
mod game_state;
mod entity;
mod direction;

use game_state::GameState;

use settings::meta::*;
use settings::game::BG_COLOR;

pub fn run() -> GameResult<()> {
  let mut ctx: Context = ggez::ContextBuilder::new(
    TITLE, AUTHOR
  ).window_setup(
    ggez::conf::WindowSetup::default().title(WINDOW_TITLE)
  ).window_mode(
    ggez::conf::WindowMode::default().dimensions(
      WINDOW_SIZE.w as u32,
      WINDOW_SIZE.h as u32
    )
  ).build()?;

  graphics::set_background_color(&mut ctx, BG_COLOR.into());
  let mut state: GameState = GameState::new();
  return event::run(&mut ctx, &mut state);
}
