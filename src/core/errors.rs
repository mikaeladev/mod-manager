use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use eframe::Error as EframeError;
use toml::de::Error as DeError;
use toml::ser::Error as SerError;
use winit::error::EventLoopError;

#[derive(Debug)]
pub enum CoreError {
  DeError(DeError),
  EframeError(EframeError),
  EventLoopError(EventLoopError),
  IoError(IoError),
  MissingConfigHome,
  SerError(SerError),
}

impl Error for CoreError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::DeError(err) => Some(err),
      Self::EframeError(err) => Some(err),
      Self::EventLoopError(err) => Some(err),
      Self::IoError(err) => Some(err),
      Self::SerError(err) => Some(err),
      _ => None,
    }
  }
}

impl fmt::Display for CoreError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let err_msg = match self {
      Self::DeError(err) => format!("deserialization error: {err}"),
      Self::EframeError(err) => format!("eframe error: {err}"),
      Self::EventLoopError(err) => format!("event loop error: {err}"),
      Self::IoError(err) => format!("io error: {err}"),
      Self::MissingConfigHome => String::from("missing config home"),
      Self::SerError(err) => format!("serialization error: {err}"),
    };

    write!(f, "{err_msg}")
  }
}

/// A simple macro for implementing [`From<T>`] for `CoreError`.
macro_rules! impl_from_for_error {
  ($from_struct: ident) => {
    impl From<$from_struct> for CoreError {
      fn from(value: $from_struct) -> Self {
        Self::$from_struct(value)
      }
    }
  };
}

impl_from_for_error!(DeError);
impl_from_for_error!(EframeError);
impl_from_for_error!(EventLoopError);
impl_from_for_error!(IoError);
impl_from_for_error!(SerError);
