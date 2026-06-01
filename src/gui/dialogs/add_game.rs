use std::path::PathBuf;
use std::sync::Arc;

use egui::{
  CentralPanel, Frame, Margin, Panel, TextBuffer, Ui, ViewportBuilder,
  ViewportClass, ViewportCommand, ViewportId,
};

use crate::core::games::Game;

use crate::gui::app::{App, AppConfig, AppState};
use crate::gui::root::RootTab;

use crate::utils::mutex::MutexGuard;

use super::Dialog;

pub struct AddGameDialog<'a> {
  pub app: &'static App,

  config: MutexGuard<'a, AppConfig>,
  state: MutexGuard<'a, AppState>,
}

impl<'a> AddGameDialog<'a> {
  pub fn new(app: &'static App) -> Self {
    Self {
      app,
      config: app.config.lock(),
      state: app.state.lock(),
    }
  }

  pub fn ui(mut self, ui: &mut Ui, class: ViewportClass) {
    self.bottom_panel(ui, class);
    self.central_panel(ui);
  }

  fn bottom_panel(&mut self, ui: &mut Ui, class: ViewportClass) {
    Panel::bottom("add_game_dialog_buttons")
      .frame(Frame::new().inner_margin(Margin::same(12)))
      .show_inside(ui, |ui| {
        ui.horizontal(|ui| {
          if ui.button("Discard").clicked() {
            Self::close_window(ui, class);
          };

          if ui.button("Confirm").clicked() {
            if let Some(input) = self.state.add_game_dialog.take() {
              let game: Arc<Game> = Arc::new(input.into());

              self.state.selected_game = Some(game.id);
              self.state.selected_tab = RootTab::Game;

              self.config.games.insert_from(game);

              Self::close_window(ui, class);
            }
          };
        });
      });
  }

  fn central_panel(&mut self, ui: &mut Ui) {
    CentralPanel::default()
      .frame(Frame::new().inner_margin(Margin::same(12)))
      .show_inside(ui, |ui| {
        let state = &mut *self.state;

        let input = match &mut state.add_game_dialog {
          Some(input) => input,
          None => &mut Default::default(),
        };

        ui.horizontal(|ui| {
          ui.label("Name:");
          ui.text_edit_singleline(&mut input.name);
        });

        ui.horizontal(|ui| {
          ui.label("Staging Directory:");
          ui.horizontal(|ui| {
            ui.add_enabled_ui(!state.file_dialog_open, |ui| {
              ui.text_edit_singleline(&mut input.staging_dir);

              if ui.button("Select").clicked() {
                state.file_dialog_open = true;

                let state = Arc::clone(&self.app.state);

                self.app.runtime.spawn(async move {
                  let value = rfd::FileDialog::new().pick_folder();
                  let state = &mut state.lock();

                  if let Some(path) = value
                    && let Some(input) = state.add_game_dialog.as_mut()
                  {
                    input.staging_dir.replace_with(&path.to_string_lossy());
                  };

                  state.file_dialog_open = false;
                });
              };
            })
          })
        });
      });
  }
}

impl Dialog for AddGameDialog<'_> {
  fn show_viewport_deferred(app: &'static App, ui: &mut Ui) {
    ui.show_viewport_deferred(
      ViewportId::from_hash_of("add_game_dialog"),
      ViewportBuilder::default()
        .with_title("Add Game")
        .with_app_id(App::APP_WINDOW_ID)
        .with_inner_size([400., 200.]),
      move |ui, class| {
        Self::new(app).ui(ui, class);

        if ui.input(|i| i.viewport().close_requested()) {
          app.state.lock().add_game_dialog = None;
        };
      },
    )
  }

  fn close_window(ui: &mut Ui, class: ViewportClass) {
    if class == ViewportClass::EmbeddedWindow {
      todo!("embedded windows are currently unsupported")
    } else {
      ui.send_viewport_cmd(ViewportCommand::Close);
    }
  }
}

#[derive(Default, PartialEq, Eq)]
pub struct AddGameDialogInput {
  pub name: String,
  pub staging_dir: String,
  pub steam_id: Option<u32>,
}

impl Into<Game> for AddGameDialogInput {
  fn into(self) -> Game {
    Game::new(
      None,
      self.name,
      PathBuf::from(self.staging_dir),
      self.steam_id,
    )
  }
}
