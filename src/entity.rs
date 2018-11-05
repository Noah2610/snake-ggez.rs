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
use ::color::Color;
use ::direction::Direction;

use ::settings::entity::player;

pub trait Entity: Mask {
  fn color(&self) -> Color;

  fn move_by(&mut self, point_incr: &Point) {
    self.point_mut().add(point_incr);
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_color(ctx, self.color().into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.rectangle());
    return Ok(());
  }
}

pub struct Player {
  point:        Point,
  size:         Size,
  origin:       Origin,
  color:        Color,
  direction:    Direction,
  last_step:    Direction,
  last_step_at: Instant
}

impl Player {
  pub fn new(point: Point) -> Self {
    Self {
      point:        point,
      size:         player::SIZE,
      origin:       player::ORIGIN,
      color:        player::COLOR,
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
}

impl Mask for Player {
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

impl Entity for Player {
  fn color(&self) -> Color {
    self.color
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    if Instant::now() - self.last_step_at < Duration::from_millis(player::STEP_EVERY_MS) {
      return Ok(());
    }

    let point_incr: Point = match self.direction {
      Direction::Up    => Point::new(
        0.0,
        -player::STEP_SIZE
      ),
      Direction::Down  => Point::new(
        0.0,
        player::STEP_SIZE
      ),
      Direction::Left  => Point::new(
        -player::STEP_SIZE,
        0.0
      ),
      Direction::Right => Point::new(
        player::STEP_SIZE,
        0.0
      )
    };
    self.move_by(&point_incr);
    self.last_step = self.direction.clone();
    self.last_step_at = Instant::now();
    return Ok(());
  }
}
