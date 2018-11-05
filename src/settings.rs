pub mod meta {
  use ::geo::Size;

  pub const TITLE:        &str = "Snake";
  pub const AUTHOR:       &str = "Noah R";
  pub const WINDOW_TITLE: &str = "Snake!";
  pub const WINDOW_SIZE:  Size   = Size { w: 600.0, h: 600.0 };
}

pub mod game {
  use ::color::Color;

  pub const FPS:                f32   = 30.0;
  pub const UPDATE_INTERVAL_MS: u64   = (1.0 / FPS * 1000.0) as u64;
  pub const BG_COLOR:           Color = [0.5, 0.5, 0.5, 1.0];
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

    const SIZE_SQ:           f32    = 32.0;
    pub const SIZE:          Size   = Size { w: SIZE_SQ, h: SIZE_SQ };
    pub const ORIGIN:        Origin = Origin::TopLeft;
    pub const COLOR:         Color  = [0.25, 0.75, 0.5, 1.0];
    pub const STEP_SIZE:     f32    = SIZE_SQ;
    pub const STEP_EVERY_MS: u64    = 400;
  }
}
