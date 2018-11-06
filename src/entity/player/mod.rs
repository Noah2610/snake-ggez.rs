use ::std::time::{Instant, Duration};
use ::ggez::{
  Context,
  GameResult,
  graphics
};
use ::geo::{
  Point,
  Size,
  mask::{
    Mask,
    Origin
  }
};
use super::Entity;
use ::color::Color;
use ::direction::Direction;
use ::grid::{Grid, Cell};
use ::settings::entity::player::*;

mod head;
mod body;

use self::head::Head;
use self::body::Body;

pub struct Player<'a> {
  grid:         &'a Grid,
  cell:         &'a Cell,
  head:         Head,
  bodies:       Vec<Body>,
  direction:    Direction,
  last_step:    Direction,
  last_step_at: Instant
}

impl<'a> Player<'a> {
  pub fn new(point: Point, grid: &'a Grid) -> Self {
    let mut bodies: Vec<Body> = Vec::new();
    for i in (1 ..= INITIAL_BODIES) {
      let body_point: Point = Point::new(point.x - SIZE_SQ * (i as f32), point.y);
      bodies.push(Body::new(body_point));
    };
    let head: Head = Head::new(point);
    let cell: &Cell = grid.get_cell_for(&point);
    Self {
      grid:         grid,
      cell:         cell,
      head:         head,
      bodies:       bodies,
      direction:    Direction::Right,
      last_step:    Direction::Right,
      last_step_at: Instant::now()
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    if self.direction.is_opposite(&direction) || self.direction != self.last_step {
      return;
    }
    self.direction = direction;
  }

  pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if Instant::now() - self.last_step_at < Duration::from_millis(STEP_EVERY_MS) {
      return Ok(());
    }
    self.step(_ctx)?;
    return Ok(());
  }

  fn step(&mut self, _ctx: &mut Context) -> GameResult<()> {
    let point_incr: Point = match self.direction {
      Direction::Up    => Point::new(
        0.0,
        -STEP_SIZE
      ),
      Direction::Down  => Point::new(
        0.0,
        STEP_SIZE
      ),
      Direction::Left  => Point::new(
        -STEP_SIZE,
        0.0
      ),
      Direction::Right => Point::new(
        STEP_SIZE,
        0.0
      )
    };

    for i in (0 .. self.bodies.len()).rev() {
      let new_point: Point = if i > 0 {
        self.bodies[i - 1].point().clone()
      } else {
        self.head.point().clone()
      };
      self.bodies[i].move_to(&new_point);
    }

    let mut new_point: Point = self.head.point().clone();
    new_point.add(&point_incr);

    self.head.move_to(&new_point);

    // self.head.move_by(&point_incr);

    self.last_step = self.direction.clone();
    self.last_step_at = Instant::now();
    return Ok(());
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    for body in &mut self.bodies {
      body.draw(ctx)?;
    }
    self.head.draw(ctx)?;
    return Ok(());
  }
}
