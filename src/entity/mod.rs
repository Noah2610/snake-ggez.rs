use ::ggez::{
  Context,
  GameResult,
  graphics
};

use ::geo::{
  Point,
  mask::Mask
};
use ::color::Color;

pub mod player;

pub trait Entity: Mask {
  fn color(&self) -> Color;

  fn move_by(&mut self, point_incr: &Point) {
    self.point_mut().add(point_incr);
  }

  fn move_to(&mut self, new_point: &Point) {
    self.point_mut().set(new_point);
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_color(ctx, self.color().into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.rectangle())?;
    return Ok(());
  }
}
