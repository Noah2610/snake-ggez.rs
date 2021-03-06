pub mod mask;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  pub x: f32,
  pub y: f32
}

impl Point {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn combine(points: Vec<&Point>) -> Point {
    let mut point_acc: Point = Point::new(0.0, 0.0);
    for point in points {
      point_acc.add(point);
    }
    return point_acc;
  }

  pub fn add(&mut self, point: &Point) {
    self.x += point.x;
    self.y += point.y;
  }

  pub fn set(&mut self, point: &Point) {
    self.x = point.x;
    self.y = point.y;
  }

  // gotta learn how to write `into()` methods
  pub fn point2(&self) -> ::ggez::graphics::Point2 {
    ::ggez::graphics::Point2::new(self.x, self.y)
  }
}

pub struct Size {
  pub w: f32,
  pub h: f32
}

impl Size {
  pub fn new(w: f32, h: f32) -> Self {
    Self { w, h }
  }

  pub fn center(&self) -> Point {
    Point::new(self.w / 2.0, self.h / 2.0)
  }
}
