extern crate ggez;

use std::time::{Instant, Duration};
use ggez::{
  GameResult,
  Context,
  event,
  graphics
};

mod settings;
mod color;
mod geo;

use settings::meta::*;
use settings::game::*;

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

  let mut state: GameState = GameState::new();

  return event::run(&mut ctx, &mut state);
}

struct GameState {
  last_update: Instant
}

impl GameState {
  pub fn new() -> Self {
    Self {
      last_update: Instant::now()
    }
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if Instant::now() - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }

    // self.player.update(_ctx)?;

    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    // self.player.draw(ctx)?;

    graphics::present(ctx);
    ggez::timer::yield_now();
    return Ok(());
  }
}
