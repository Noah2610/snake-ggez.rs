pub mod meta {
  use ::geo::Size;

  pub const WINDOW_SIZE:  Size = Size { w: 600.0, h: 600.0 };
  pub const TITLE:        &str = "Snake";
  pub const AUTHOR:       &str = "Noah R";
  pub const WINDOW_TITLE: &str = "Snake!";
}

pub mod game {
  use ::color::Color;
  use ::geo::{
    Size,
    mask::Origin
  };

  pub const FPS:                f32    = 30.0;
  pub const UPDATE_INTERVAL_MS: u64    = (1.0 / FPS * 1000.0) as u64;
  pub const BG_COLOR:           Color  = [0.5, 0.5, 0.5, 1.0];
  pub const ORIGIN:             Origin = Origin::TopLeft;

  pub const CELL_SIZE_SQ:       f32    = 32.0;
  pub const CELL_SIZE:          Size   = Size { w: CELL_SIZE_SQ, h: CELL_SIZE_SQ };

  pub const SCORE_FONT_SIZE:    u32    = 24;
}

pub mod controls {
  use ::ggez::event::Keycode;

  pub const QUIT:  Keycode = Keycode::Escape;

  pub const UP:    Keycode = Keycode::W;
  pub const DOWN:  Keycode = Keycode::S;
  pub const LEFT:  Keycode = Keycode::A;
  pub const RIGHT: Keycode = Keycode::D;
}

pub mod entity {
  pub mod player {
    use ::color::Color;
    use ::geo::{
      Size,
      mask::Origin
    };

    pub const SIZE_SQ:        f32    = super::super::game::CELL_SIZE_SQ;
    pub const SIZE:           Size   = super::super::game::CELL_SIZE;
    pub const ORIGIN:         Origin = Origin::TopLeft;
    pub const COLOR:          Color  = [0.25, 0.75, 0.5, 1.0];
    pub const BODY_COLOR:     Color  = [0.1, 0.5, 0.25, 1.0];
    pub const STEP_SIZE:      f32    = SIZE_SQ;
    pub const STEP_EVERY_MS:  u64    = 200;
    pub const INITIAL_BODIES: u32    = 5;
  }

  pub mod food {
    use ::color::*;
    use ::geo::{
      Size,
      mask::Origin
    };

    const PADDING_SQ:  f32    = 8.0;
    const SIZE_SQ:     f32    = super::super::game::CELL_SIZE_SQ;
    pub const PADDING: Size   = Size { w: PADDING_SQ, h: PADDING_SQ };
    pub const SIZE:    Size   = Size { w: SIZE_SQ, h: SIZE_SQ };
    pub const ORIGIN:  Origin = Origin::TopLeft;
    pub const COLOR:   Color  = RED;

    pub const CHECK_SPAWN_EVERY_MS: u64   = super::player::STEP_EVERY_MS;
    pub const SPAWN_CHANCE:         f32   = 0.1;
    pub const SPAWN_TRIES:          usize = 50;
  }
}
