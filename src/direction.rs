#[derive(Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

use Direction::*;

impl Direction {
  pub fn is_opposite(&self, other: &Direction) -> bool {
    match self {
      Up    => if let Down  = other { true } else { false },
      Down  => if let Up    = other { true } else { false },
      Left  => if let Right = other { true } else { false },
      Right => if let Left  = other { true } else { false },
    }
  }
}
