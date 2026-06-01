use std::path::{Path, PathBuf};
use std::{fs, io};

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils;

pub type ModId = uuid::Uuid;

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Mod {
  pub id: ModId,
  pub name: String,
  pub source: SourceKind,
  pub version: Option<String>,
  pub installed_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

impl Mod {
  pub fn new(
    id: impl Into<Option<ModId>>,
    name: impl ToString,
    source: SourceKind,
    version: impl Into<Option<String>>,
  ) -> Self {
    Self {
      id: id.into().unwrap_or_else(|| ModId::new_v4()),
      name: name.to_string(),
      source,
      version: version.into(),
      installed_at: Utc::now().naive_utc(),
      ..Default::default()
    }
  }

  pub fn from_local(
    name: impl ToString,
    source_path: impl AsRef<Path>,
    version: impl Into<Option<String>>,
    staging_dir: &PathBuf,
  ) -> io::Result<Self> {
    let source_path = source_path.as_ref();

    if !source_path.exists() {
      return Err(io::ErrorKind::NotFound.into());
    }

    let id = ModId::default();
    let dest_dir = staging_dir.join(id.to_string());

    if source_path.is_file() {
      let file_name = source_path.file_name().unwrap();
      utils::fs::ensure_dir(&dest_dir)?;
      fs::copy(&source_path, dest_dir.join(file_name))?;
    } else if source_path.is_dir() {
      utils::fs::copy_dir_all(source_path, &dest_dir)?;
    } else {
      return Err(io::ErrorKind::InvalidInput.into());
    }

    let source = SourceKind::Local(dest_dir);

    Ok(Self::new(id, name, source, version))
  }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SourceKind {
  /// Local file system path.
  Local(PathBuf),
  /// Remote resource on the web.
  Remote(String),
  /// Default fallback value.
  #[default]
  Unknown,
}
