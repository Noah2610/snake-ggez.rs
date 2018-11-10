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
use ::settings::entity::player::*;

mod head;
mod body;

use self::head::Head;
use self::body::Body;

pub struct Player {
  pub head:   Head,
  pub bodies: Vec<Body>,
  direction:  Direction,
  last_step:  Direction
}

impl Player {
  pub fn new(point: Point) -> Self {
    let mut bodies: Vec<Body> = Vec::new();
    for i in (1 ..= INITIAL_BODIES) {
      let body_point: Point = Point::new(point.x - SIZE_SQ * (i as f32), point.y);
      bodies.push(Body::new(body_point));
    };
    let head: Head = Head::new(point);
    Self {
      head:         head,
      bodies:       bodies,
      direction:    Direction::Right,
      last_step:    Direction::Right
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    if self.direction.is_opposite(&direction) || self.direction != self.last_step {
      return;
    }
    self.direction = direction;
  }

  pub fn step(&mut self) -> GameResult<()> {
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

    return Ok(());
  }

  pub fn add_body(&mut self) {
    let point: Point = self.bodies.last().expect("Player/Snake should always have at least one Body.").point().clone();
    self.bodies.push(
      Body::new(point)
    );
  }

  pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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
