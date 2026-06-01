use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fmt, fs};

use ahash::AHashMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::game::{Game, GameId};

/// A map of `Game`s.
///
/// Serialises to a map of `staging_dir` paths.
#[derive(Default)]
pub struct GameIndex(AHashMap<GameId, Arc<Game>>);

impl GameIndex {
  #[inline]
  pub fn new() -> Self {
    Self(AHashMap::new())
  }

  #[inline]
  pub fn insert_from(&mut self, value: Arc<Game>) -> Option<Arc<Game>> {
    self.insert(value.id, value)
  }
}

impl Deref for GameIndex {
  type Target = AHashMap<GameId, Arc<Game>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for GameIndex {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

struct GameIndexVisitor;

impl<'de> Visitor<'de> for GameIndexVisitor {
  type Value = GameIndex;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "a map of uuid-path pairs")
  }

  fn visit_map<A: MapAccess<'de>>(
    self,
    mut access: A,
  ) -> Result<Self::Value, A::Error> {
    let mut map = AHashMap::with_capacity(access.size_hint().unwrap_or(0));

    while let Some((uuid, path_buf)) = access.next_entry::<GameId, PathBuf>()? {
      match fs::read(path_buf) {
        Ok(bytes) => match toml::from_slice(bytes.as_slice()) {
          Ok(value) => {
            map.insert(uuid, value);
          }
          Err(err) => {
            eprintln!("failed to parse game config: {err}");
            continue;
          }
        },
        Err(err) => {
          eprintln!("failed to read game config: {err}");
          continue;
        }
      }
    }

    Ok(GameIndex(map))
  }
}

impl Serialize for GameIndex {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    let mut map = ser.serialize_map(Some(self.0.len()))?;

    for (key, value) in &self.0 {
      map.serialize_entry(key, &value.staging_dir)?;
    }

    map.end()
  }
}

impl<'de> Deserialize<'de> for GameIndex {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    de.deserialize_map(GameIndexVisitor)
  }
}
