mod add_game;

use egui::{Ui, ViewportClass};

use super::app::App;

pub trait Dialog {
  fn show_viewport_deferred(app: &'static App, ui: &mut Ui);

  fn close_window(ui: &mut Ui, class: ViewportClass);
}

pub use self::add_game::*;
