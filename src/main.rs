extern crate snake;

use std::process::exit;

fn main() {
  if let Err(err) = snake::run() {
    eprintln!("Error: {}", err);
    exit(1);
  }
}
