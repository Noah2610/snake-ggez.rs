pub mod meta {
  use ::geo::Size;

  pub const TITLE:        &str = "Snake";
  pub const AUTHOR:       &str = "Noah R";
  pub const WINDOW_TITLE: &str = "Snake!";
  pub const WINDOW_SIZE:  Size   = Size { w: 600.0, h: 600.0 };
}

pub mod game {
  pub const FPS:                f32 = 30.0;
  pub const UPDATE_INTERVAL_MS: u64 = (1.0 / FPS * 1000.0) as u64;
}
