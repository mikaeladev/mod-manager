mod core;
mod gui;
mod utils;

use core::errors::CoreError;

fn main() -> Result<(), CoreError> {
  gui::app::run()
}
