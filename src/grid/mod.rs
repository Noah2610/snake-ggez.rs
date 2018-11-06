use ::settings::grid::*;

mod cell;

use self::cell::Cell;

use ::geo::{
  Size
};

pub struct Grid {
  size:      Size,
  cell_size: Size,
  cells:     Vec<Cell>
}

impl Grid {
  pub fn new() -> Self {
    let grid: Self = Self {
      size:      SIZE,
      cell_size: CELL_SIZE,
      cells:     Vec::new()
    };
    grid.generate_cells();
    return grid;
  }

  pub fn generate_cells(&mut self) {

  }
}
