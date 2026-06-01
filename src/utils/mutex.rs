use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use egui::mutex::MutexGuard;

/// Provides interior mutability.
///
/// This is a thin wrapper around [`egui::mutex::Mutex`].
#[derive(Clone, Default)]
pub struct Mutex<T>(egui::mutex::Mutex<T>);

impl<T: Serialize> Serialize for Mutex<T> {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    self.0.lock().serialize(ser)
  }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mutex<T> {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    Ok(Mutex::new(T::deserialize(de)?))
  }
}

impl<T> Mutex<T> {
  #[inline(always)]
  pub fn new(value: T) -> Self {
    Self(egui::mutex::Mutex::new(value))
  }

  /// Try to acquire the lock.
  ///
  /// # Panics
  ///
  /// Will panic in debug builds if the lock can't be acquired within 10
  /// seconds.
  #[inline(always)]
  #[cfg_attr(debug_assertions, track_caller)]
  pub fn lock(&self) -> MutexGuard<'_, T> {
    self.0.lock()
  }
}
