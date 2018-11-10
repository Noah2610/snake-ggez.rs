use ::ggez::{
  Context,
  GameResult,
  graphics
};

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

  fn rectangle(&self) -> graphics::Rect {
    let mut point: Point = self.top_left();
    point.add(
      &Point::new(self.padding.w, self.padding.h)
    );
    let size: Size = Size::new(
      self.size().w - (self.padding.w * 2.0),
      self.size().h - (self.padding.h * 2.0)
    );
    return [
      point.x, point.y,
      size.w,     size.h
    ].into();
  }
}

impl Entity for Food {
  fn color(&self) -> Color {
    self.color
  }
}
