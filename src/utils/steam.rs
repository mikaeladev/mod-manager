use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use steamlocate::SteamDir;

/// A wrapper around [`SteamDir`] that implements [`serde`] traits.
pub struct SteamDirWrapper(Option<SteamDir>);

impl Default for SteamDirWrapper {
  fn default() -> Self {
    Self(SteamDir::locate().ok())
  }
}

impl Serialize for SteamDirWrapper {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    match &self.0 {
      Some(steamdir) => ser.serialize_some(&steamdir.path().to_str()),
      None => ser.serialize_none(),
    }
  }
}

impl<'de> Deserialize<'de> for SteamDirWrapper {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    if let Some(path_str) = Option::<&str>::deserialize(de)? {
      let path_buf = PathBuf::from(path_str);

      match SteamDir::from_dir(path_buf.as_path()) {
        Ok(steamdir) => Ok(SteamDirWrapper(Some(steamdir))),
        Err(err) => {
          eprintln!("error using configured steam dir path: {err}");
          Ok(SteamDirWrapper(None))
        }
      }
    } else {
      Ok(SteamDirWrapper(None))
    }
  }
}
