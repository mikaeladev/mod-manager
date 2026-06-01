use std::path::PathBuf;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::core::deployers::{Deployer, DeployerId};
use crate::core::mods::{Mod, ModId};

pub type GameId = uuid::Uuid;

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Game {
  pub id: GameId,
  pub name: String,
  pub mods: AHashMap<ModId, Mod>,
  pub deployers: AHashMap<DeployerId, Deployer>,
  pub staging_dir: PathBuf,
  pub steam_id: Option<u32>,
}

impl Game {
  pub fn new(
    id: impl Into<Option<ModId>>,
    name: String,
    staging_dir: PathBuf,
    steam_id: impl Into<Option<u32>>,
  ) -> Self {
    Self {
      id: id.into().unwrap_or_else(|| GameId::new_v4()),
      name,
      staging_dir,
      steam_id: steam_id.into(),
      ..Default::default()
    }
  }
}
