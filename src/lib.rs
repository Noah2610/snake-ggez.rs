extern crate ggez;

use ggez::{
  GameResult,
  Context
};

mod settings;
mod color;
mod geo;

use settings::meta::*;

pub fn run() -> GameResult<()> {
  let ctx: Context = ggez::ContextBuilder::new(
    TITLE, AUTHOR
  ).window_setup(
    ggez::conf::WindowSetup::default().title(WINDOW_TITLE)
  ).window_mode(
    ggez::conf::WindowMode::default().dimensions(
      WINDOW_SIZE.w as u32,
      WINDOW_SIZE.h as u32
    )
  ).build()?;

  return Ok(());
}
