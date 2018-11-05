extern crate ggez;

use std::time::{Instant, Duration};
use ggez::{
  GameResult,
  Context,
  graphics,
  event,
  event::{Keycode}
};

mod settings;
mod color;
mod geo;
mod entity;
mod direction;

use geo::{
  Point
};
use entity::{
  Entity,
  player::Player
};
use direction::Direction;

use settings::meta::*;
use settings::game::*;
use settings::controls;

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

struct GameState {
  player:      Player,
  last_update: Instant
}

impl GameState {
  pub fn new() -> Self {
    let mut center: Point = WINDOW_SIZE.center();
    center.add(&Point::new(
        (settings::entity::player::SIZE.w / 2.0) * -1.0,
        (settings::entity::player::SIZE.h / 2.0) * -1.0
    ));
    Self {
      player: Player::new(center),
      last_update: Instant::now()
    }
  }
}

impl event::EventHandler for GameState {
  fn key_down_event(
    &mut self,
    ctx:     &mut Context,
    keycode: Keycode,
    _keymod: event::Mod,
    repeat:  bool,
    ) {
    if repeat { return; }
    match keycode {
      controls::QUIT  => ctx.quit().expect("Couldn't quit Context"),
      controls::UP    => self.player.set_direction(Direction::Up),
      controls::DOWN  => self.player.set_direction(Direction::Down),
      controls::LEFT  => self.player.set_direction(Direction::Left),
      controls::RIGHT => self.player.set_direction(Direction::Right),
      _               => ()
    }
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if Instant::now() - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }

    self.player.update(_ctx)?;

    self.last_update = Instant::now();
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    self.player.draw(ctx)?;

    graphics::present(ctx);
    ggez::timer::yield_now();
    return Ok(());
  }
}
