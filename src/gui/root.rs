use egui::{CentralPanel, ComboBox, Frame, Margin, Panel, Ui};
use serde::{Deserialize, Serialize};

use crate::utils::mutex::MutexGuard;

use super::app::{App, AppConfig, AppState};
use super::dialogs::{AddGameDialog, Dialog};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum RootTab {
  #[default]
  None,
  Game,
  Mods,
  Deployers,
}

pub struct Root<'a> {
  pub app: &'static App,

  config: MutexGuard<'a, AppConfig>,
  state: MutexGuard<'a, AppState>,
}

impl<'a> Root<'a> {
  pub fn new(app: &'static App) -> Self {
    Self {
      app,
      config: app.config.lock(),
      state: app.state.lock(),
    }
  }

  pub fn ui(mut self, ui: &mut Ui) {
    self.top_panel(ui);
    self.central_panel(ui);
    self.dialogs(ui);
  }

  fn top_panel(&mut self, ui: &mut Ui) {
    Panel::top("root_top_panel")
      .frame(Frame::new().inner_margin(Margin::same(12)))
      .show_inside(ui, |ui| {
        ui.horizontal_centered(|ui| {
          // button for adding/importing games
          ui.add_enabled_ui(!self.state.add_game_dialog.is_some(), |ui| {
            ui.menu_button("Add Game", |ui| {
              if ui.button("Add Manually").clicked() {
                self.state.add_game_dialog = Some(Default::default());
              };

              if ui.button("Import from Steam").clicked() {
                todo!();
              };
            });
          });

          ui.separator();

          // combobox for selecting games
          ui.add_enabled_ui(!self.config.games.is_empty(), |ui| {
            let combobox = ComboBox::from_id_salt("select_game_combobox")
              .width(128.)
              .selected_text(
                if ui.is_enabled()
                  && let Some(id) = self.state.selected_game
                  && let Some(game) = self.config.games.get(&id)
                {
                  game.name.as_str()
                } else {
                  "Select a game"
                },
              );

            combobox.show_ui(ui, |ui| {
              for game in self.config.games.values() {
                let button = ui.selectable_value(
                  &mut self.state.selected_game,
                  Some(game.id),
                  &game.name,
                );

                if button.clicked() && self.state.selected_tab == RootTab::None
                {
                  self.state.selected_tab = RootTab::Game
                }
              }
            });
          });

          ui.separator();

          // tab buttons
          ui.add_enabled_ui(self.state.selected_game.is_some(), |ui| {
            ui.horizontal(|ui| {
              if ui.button("Game").clicked() {
                self.state.selected_tab = RootTab::Game;
              };
              if ui.button("Mods").clicked() {
                self.state.selected_tab = RootTab::Mods;
              };
              if ui.button("Deployers").clicked() {
                self.state.selected_tab = RootTab::Deployers;
              };
            });
          });
        });
      });
  }

  fn central_panel(&mut self, ui: &mut Ui) {
    CentralPanel::default()
      .frame(Frame::new().inner_margin(Margin::same(12)))
      .show_inside(ui, |ui| {
        let mut selected_game = None;

        if let Some(game_id) = self.state.selected_game {
          if let Some(game) = self.config.games.get(&game_id) {
            selected_game = Some(game);

            if self.state.selected_tab == RootTab::None {
              self.state.selected_tab = RootTab::Game;
            }
          } else {
            // todo: swap for some kind of debug-only logging lib
            eprintln!("warning: selected game not found");
            self.state.selected_tab = RootTab::None;
          }
        };

        if selected_game.is_none() {
          ui.heading("No game selected");
          return;
        }

        let selected_game = selected_game.unwrap();

        match self.state.selected_tab {
          RootTab::Game => {
            ui.heading(selected_game.name.as_str());
          }
          _ => todo!(),
        };
      });
  }

  fn dialogs(&self, ui: &mut Ui) {
    if self.state.add_game_dialog.is_some() {
      AddGameDialog::show_viewport_deferred(self.app, ui);
    };
  }
}
