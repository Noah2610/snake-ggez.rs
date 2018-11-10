use ::std::time::{Instant, Duration};
use ::rand::prelude::*;

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
  mask::{Mask, Origin}
};
use ::entity::{
  Entity,
  player::Player,
  food::Food
};
use ::direction::Direction;

use ::settings::game::*;
use ::settings::meta::WINDOW_SIZE;
use ::settings::controls;
use ::settings::entity::{
  player::{
    self,
    STEP_EVERY_MS
  },
  food
};

pub struct GameState {
  point:                 Point,
  size:                  Size,
  origin:                Origin,
  player:                Player,
  foods:                 Vec<Food>,
  running:               bool,
  last_player_move:      Instant,
  last_check_spawn_food: Instant,
  last_update:           Instant
}

impl GameState {
  pub fn new() -> Self {
    Self {
      point:                 Point::new(0.0, 0.0),
      size:                  WINDOW_SIZE,
      origin:                ORIGIN,
      player:                Player::new(Point::new(0.0, 0.0)),
      foods:                 Vec::new(),
      running:               true,
      last_player_move:      Instant::now(),
      last_check_spawn_food: Instant::now(),
      last_update:           Instant::now()
    }
  }

  fn check_move_player(&mut self) -> GameResult<()> {
    if Instant::now() - self.last_player_move < Duration::from_millis(STEP_EVERY_MS) {
      return Ok(());
    }
    self.move_player()?;
    self.last_player_move = Instant::now();
    return Ok(());
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
    let cols: u32 = (WINDOW_SIZE.w / CELL_SIZE.w) as u32;
    let rows: u32 = (WINDOW_SIZE.h / CELL_SIZE.h) as u32;
    if center.y < 0.0 {
      self.player.head.move_to(
        &Point::new(top_left.x, player::SIZE.h * (cols as f32) )
      );
    } else if center.y > WINDOW_SIZE.h {
      self.player.head.move_to(
        &Point::new(top_left.x, 0.0)
      );
    } else if center.x < 0.0 {
      self.player.head.move_to(
        &Point::new(player::SIZE.w * (rows as f32), top_left.y)
      );
    } else if center.x > WINDOW_SIZE.w {
      self.player.head.move_to(
        &Point::new(0.0, top_left.y)
      );
    }

    // ... with food
    let food_opt: Option<usize> = self.foods.iter().enumerate()
      .find_map( |(index, food)|
                 if food.intersects(&self.player.head) {
                   Some(index)
                 } else { None }
      );
    if let Some(index) = food_opt {
      self.player.add_body();
      self.foods.remove(index);
    }

    return Ok(());
  }

  fn game_over(&mut self) {
    self.running = false;
  }

  fn check_spawn_food(&mut self) -> GameResult<()> {
    if Instant::now() - self.last_check_spawn_food < Duration::from_millis(food::CHECK_SPAWN_EVERY_MS) {
      return Ok(());
    }
    if thread_rng().gen::<f32>() <= food::SPAWN_CHANCE {
      self.spawn_food()?;
    }
    self.last_check_spawn_food = Instant::now();
    return Ok(());
  }

  fn spawn_food(&mut self) -> GameResult<()> {
    if let Some(point) = self.get_new_food_point() {
      self.foods.push(
        Food::new(point)
      );
    } else {
      println!("Couldn't spawn a Food item!");
    }
    return Ok(());
  }

  fn get_new_food_point(&self) -> Option<Point> {
    let cols: u32 = (WINDOW_SIZE.w / CELL_SIZE.w) as u32;
    let rows: u32 = (WINDOW_SIZE.h / CELL_SIZE.h) as u32;
    (0 .. food::SPAWN_TRIES).find_map( |_i| {
      let point: Point = Point::new(
        thread_rng().gen_range::<u32>(0, cols) as f32 * CELL_SIZE.w,
        thread_rng().gen_range::<u32>(0, rows) as f32 * CELL_SIZE.h
      );
      if self.is_cell_free(&point) {
        Some(point)
      } else {
        None
      }
    })
  }

  fn is_cell_free(&self, point: &Point) -> bool {
    self.player.head.point() != point                                &&
      !self.player.bodies.iter().any( |body| body.point() == point ) &&
      !self.foods.iter().any( |food| food.point() == point )
  }
}

impl Mask for GameState {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
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

    self.check_move_player()?;
    self.check_spawn_food()?;

    // TODO: Updating player is no longer necessary; self handles player movement.
    //self.player.update(_ctx)?;

    self.last_update = Instant::now();
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    for food in &mut self.foods {
      food.draw(ctx)?;
    }

    self.player.draw(ctx)?;

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
