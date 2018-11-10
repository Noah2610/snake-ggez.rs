use std::time::{Instant, Duration};

use ::ggez::{
  GameResult,
  Context,
  graphics,
  event,
  event::{Keycode}
};

use ::geo::{
  Point,
  Size,
  mask::Mask
};
use ::entity::{
  Entity,
  player::Player
};
use ::direction::Direction;

use ::settings::game::*;
use ::settings::meta::WINDOW_SIZE;
use ::settings::controls;
use ::settings::entity::player::{
  self,
  STEP_EVERY_MS
};

pub struct GameState {
  point:            Point,
  size:             Size,
  player:           Player,
  running:          bool,
  last_player_move: Instant,
  last_update:      Instant
}

impl GameState {
  pub fn new() -> Self {
    let mut center: Point = WINDOW_SIZE.center();
    center.add(&Point::new(
        (::settings::entity::player::SIZE.w / 2.0) * -1.0,
        (::settings::entity::player::SIZE.h / 2.0) * -1.0
    ));
    Self {
      point:            Point::new(0.0, 0.0),
      size:             WINDOW_SIZE,
      player:           Player::new(center),
      running:          true,
      last_player_move: Instant::now(),
      last_update:      Instant::now()
    }
  }

  fn move_player(&mut self) -> GameResult<()> {
    self.player.step()?;
    // Check collision ...
    // ... with player bodies
    if self.player.bodies.iter().any( |body| self.player.head.intersects(body) ) {
      self.game_over();
    }
    // ... with walls
    let top_left: Point = self.player.head.point().clone();
    let center: Point = self.player.head.center();
    if center.y < 0.0 {
      self.player.head.move_to(&Point::new(top_left.x, self.size.h - player::SIZE.h));
    } else if center.y > WINDOW_SIZE.h {
      self.player.head.move_to(&Point::new(top_left.x, 0.0));
    } else if center.x < 0.0 {
      self.player.head.move_to(&Point::new(self.size.w - player::SIZE.w, top_left.y));
    } else if center.x > WINDOW_SIZE.w {
      self.player.head.move_to(&Point::new(0.0, top_left.y));
    }
    // ... with food (TODO)
    return Ok(());
  }

  fn game_over(&mut self) {
    self.running = false;
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
    if !self.running || Instant::now() - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }


    if Instant::now() - self.last_player_move >= Duration::from_millis(STEP_EVERY_MS) {
      self.move_player()?;
      self.last_player_move = Instant::now();
    }

    // TODO: Updating player is no longer necessary; self handles player movement.
    //self.player.update(_ctx)?;

    self.last_update = Instant::now();
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    self.player.draw(ctx)?;

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
