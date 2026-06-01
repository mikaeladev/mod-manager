use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::mods::ModId;

pub type DeployerId = uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Deployer {
  pub id: DeployerId,
  pub name: String,
  pub method: DeployerMethod,
  pub mod_order: Vec<ModId>,
  pub target_dir: PathBuf,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub enum DeployerMethod {
  #[default]
  Symlink,
  Hardlink,
  Copy,
}
