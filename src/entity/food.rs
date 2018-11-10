use ::color::Color;
use ::geo::{
  Point,
  Size,
  mask::{Mask, Origin}
};
use ::entity::Entity;

use ::settings::entity::food::*;

pub struct Food {
  point:   Point,
  size:    Size,
  padding: Size,
  origin:  Origin,
  color:   Color
}

impl Food {
  pub fn new(point: Point) -> Self {
    Self {
      point,
      size:    SIZE,
      padding: PADDING,
      origin:  ORIGIN,
      color:   COLOR
    }
  }
}

impl Mask for Food {
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

impl Entity for Food {
  fn color(&self) -> Color {
    self.color
  }
}
