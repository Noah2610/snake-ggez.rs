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

pub struct Head {
  point:        Point,
  size:         Size,
  origin:       Origin,
  color:        Color
}

impl Head {
  pub fn new(point: Point) -> Self {
    Self {
      point,
      size:   SIZE,
      origin: ORIGIN,
      color:  COLOR
    }
  }
}

impl Mask for Head {
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

impl Entity for Head {
  fn color(&self) -> Color {
    self.color
  }
}
