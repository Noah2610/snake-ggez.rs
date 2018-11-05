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
use ::settings::entity::player::*;

pub struct Body {
  point:        Point,
  size:         Size,
  origin:       Origin,
  color:        Color,
}

impl Body {
  pub fn new(point: Point) -> Self {
    Self {
      point,
      size:   SIZE,
      color:  BODY_COLOR,
      origin: ORIGIN
    }
  }
}

impl Mask for Body {
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

impl Entity for Body {
  fn color(&self) -> Color {
    self.color
  }
}
