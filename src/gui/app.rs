use std::sync::Arc;

use eframe::App as EframeApp;
use eframe::Frame as EframeFrame;
use eframe::Storage as EframeStorage;
use eframe::{CreationContext, NativeOptions};
use egui::{Theme, Ui, Vec2, ViewportBuilder};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::core::errors::CoreError;
use crate::core::games::{GameId, GameIndex};

use crate::utils::mutex::Mutex;
use crate::utils::steam::SteamDirWrapper;

use super::dialogs::AddGameDialogInput;
use super::root::{Root, RootTab};
use super::storage::Storage;

/// Initialises the GUI.
pub fn run() -> Result<(), CoreError> {
  let runtime = Runtime::new().unwrap();
  let storage = Storage::new()?;

  let native_options = NativeOptions {
    viewport: ViewportBuilder::default()
      .with_title(App::APP_NAME)
      .with_app_id(App::APP_WINDOW_ID),
    persistence_path: Some(storage.state_file.clone()),
    ..Default::default()
  };

  eframe::run_native(
    App::APP_WINDOW_ID,
    native_options,
    Box::new(|cc| {
      let app = App::new(cc, runtime, storage);
      let wrapper = AppWrapper(Box::leak(Box::new(app)));

      Ok(Box::new(wrapper))
    }),
  )?;

  Ok(())
}

pub struct App {
  pub config: Arc<Mutex<AppConfig>>,
  pub state: Arc<Mutex<AppState>>,
  pub runtime: Arc<Runtime>,
  pub storage: Arc<Storage>,
}

impl App {
  pub const APP_NAME: &str = "Mod Manager";
  pub const APP_WINDOW_ID: &str = "mod-manager";

  /// Creates a new `App`.
  ///
  /// Gets called once before the first frame.
  pub fn new(
    cc: &CreationContext<'_>,
    runtime: Runtime,
    storage: Storage,
  ) -> Self {
    cc.egui_ctx.style_mut_of(Theme::Dark, |style| {
      style.spacing.button_padding = Vec2::new(8., 6.);
      style.spacing.item_spacing = Vec2::splat(8.);
    });

    let config: AppConfig = storage.get_config();
    let mut state: AppState = storage.get_state(cc);

    if let Some(id) = state.selected_game {
      if !config.games.contains_key(&id) {
        state.selected_game = None;
        state.selected_tab = RootTab::None;
      }
    }

    Self {
      config: Arc::new(Mutex::new(config)),
      state: Arc::new(Mutex::new(state)),
      runtime: Arc::new(runtime),
      storage: Arc::new(storage),
    }
  }
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct AppConfig {
  /// Local steam API.
  #[serde(rename = "steam_path")]
  pub steam: SteamDirWrapper,

  /// A map of managed games.
  pub games: GameIndex,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct AppState {
  /// Currently selected game.
  pub selected_game: Option<GameId>,

  /// Currently selected tab.
  pub selected_tab: RootTab,

  /// Whether or not the 'Add Game' dialog is in use, with [`Some`] containing
  /// the input states.
  #[serde(skip)]
  pub add_game_dialog: Option<AddGameDialogInput>,

  /// Whether or not a [file dialog][rfd::FileDialog] is in use.
  #[serde(skip)]
  pub file_dialog_open: bool,
}

/// A thin wrapper around `App` that implements [`eframe::App`].
struct AppWrapper(&'static App);

impl EframeApp for AppWrapper {
  fn save(&mut self, eframe_storage: &mut dyn EframeStorage) {
    let app = self.0;
    app.storage.set_config(&*app.config.lock());
    app.storage.set_state(eframe_storage, &*app.state.lock());
  }

  fn ui(&mut self, ui: &mut Ui, _frame: &mut EframeFrame) {
    Root::new(&self.0).ui(ui);
  }
}
