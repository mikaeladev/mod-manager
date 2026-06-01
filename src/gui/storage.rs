use std::fs;
use std::path::PathBuf;

use eframe::CreationContext;
use eframe::Storage as EframeStorage;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::errors::CoreError;

use crate::utils;

pub struct Storage {
  /// Where downloaded mods and other assets will be cached.
  pub cache_dir: PathBuf,
  /// Where global config values will be stored.
  pub config_file: PathBuf,
  /// Where app-specific state will be stored.
  pub state_file: PathBuf,
}

impl Storage {
  #[cfg(target_os = "linux")]
  pub const STORAGE_ID: &str = "mod-manager";
  #[cfg(target_os = "macos")]
  pub const STORAGE_ID: &str = "Mod Manager";
  #[cfg(target_os = "windows")]
  pub const STORAGE_ID: &str = "ModManager";

  pub fn new() -> Result<Self, CoreError> {
    let cache_home_option = dirs::cache_dir();
    let config_home_option = dirs::config_local_dir();
    let state_home_option = dirs::state_dir();

    let config_home = config_home_option.ok_or(CoreError::MissingConfigHome)?;

    // linux:   $XDG_CACHE_HOME/mod-manager
    // macos:   $HOME/Library/Caches/Mod Manager
    // windows: {FOLDERID_LocalAppData}\ModManager\cache
    let cache_dir = if let Some(cache_home) = cache_home_option {
      cache_home.join(Self::STORAGE_ID)
    } else {
      config_home.join(format!("{}/cache", Self::STORAGE_ID))
    };

    // linux:   $XDG_CONFIG_HOME/mod-manager.toml
    // macos:   $HOME/Library/Application Support/Mod Manager/config.toml
    // windows: {FOLDERID_LocalAppData}\ModManager\config.toml
    let config_file = if cfg!(target_os = "linux") {
      config_home.join(format!("{}.toml", Self::STORAGE_ID))
    } else {
      config_home.join(format!("{}/config.toml", Self::STORAGE_ID))
    };

    // linux:   $XDG_STATE_HOME/mod-manager.ron
    // macos:   $HOME/Library/Application Support/Mod Manager/state.ron
    // windows: {FOLDERID_LocalAppData}\ModManager\state.ron
    let state_file = if let Some(state_home) = state_home_option
      && state_home != config_home
    {
      state_home.join(format!("{}.ron", Self::STORAGE_ID))
    } else {
      config_home.join(format!("{}/state.ron", Self::STORAGE_ID))
    };

    Ok(Self {
      cache_dir,
      config_file,
      state_file,
    })
  }

  /// Reads and deserialises the app config.
  pub fn get_config<T: Default + DeserializeOwned>(&self) -> T {
    match self.try_get_config() {
      Ok(value) => {
        if value.is_some() {
          eprintln!("config found!")
        } else {
          eprintln!("config not found!")
        };
        value
      }
      Err(err) => {
        eprintln!("error reading config: {err}");
        None
      }
    }
    .unwrap_or_default()
  }

  /// Attempts to read and deserialise the app config.
  fn try_get_config<T: DeserializeOwned>(
    &self,
  ) -> Result<Option<T>, CoreError> {
    if !self.config_file.exists() {
      return Ok(None);
    };

    let bytes = fs::read(&self.config_file)?;
    let value = toml::from_slice(bytes.as_slice())?;

    Ok(value)
  }

  /// Serialises and writes the app config.
  pub fn set_config(&self, value: impl Serialize) {
    match self.try_set_config(value) {
      Ok(()) => eprintln!("config saved!"),
      Err(err) => eprintln!("error saving config: {err}"),
    }
  }

  /// Attempts to serialise and write the app config.
  fn try_set_config(&self, value: impl Serialize) -> Result<(), CoreError> {
    utils::fs::ensure_parent_dir(&self.config_file)?;
    fs::write(&self.config_file, toml::to_string_pretty(&value)?)?;
    Ok(())
  }

  /// Reads and deserialises the GUI state.
  pub fn get_state<T: Default + DeserializeOwned>(
    &self,
    cc: &CreationContext<'_>,
  ) -> T {
    self.try_get_state(cc).unwrap_or_default()
  }

  /// Attempts to read and deserialise the GUI state.
  fn try_get_state<T: DeserializeOwned>(
    &self,
    cc: &CreationContext<'_>,
  ) -> Option<T> {
    let eframe_storage = cc.storage.or_else(|| {
      #[cfg(debug_assertions)]
      eprintln!("eframe storage not found, is persistence disabled?");
      None
    })?;

    eframe::get_value(eframe_storage, eframe::APP_KEY)
  }

  /// Serialises and writes the GUI state.
  pub fn set_state(
    &self,
    eframe_storage: &mut dyn EframeStorage,
    value: &impl Serialize,
  ) {
    eframe::set_value(eframe_storage, eframe::APP_KEY, value)
  }
}
