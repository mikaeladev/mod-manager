use std::path::Path;
use std::{fs, io};

pub fn ensure_dir(path: impl AsRef<Path>) -> io::Result<()> {
  let path = path.as_ref();
  if !path.try_exists()? {
    fs::create_dir_all(path)
  } else if !path.is_dir() {
    Err(io::ErrorKind::NotADirectory.into())
  } else {
    Ok(())
  }
}

pub fn ensure_parent_dir(path: impl AsRef<Path>) -> io::Result<()> {
  let path = path.as_ref();
  match path.parent() {
    None => Ok(()),
    Some(path) => ensure_dir(path),
  }
}

pub fn copy_dir_all(
  from: impl AsRef<Path>,
  to: impl AsRef<Path>,
) -> io::Result<()> {
  fs::create_dir_all(&to)?;
  for entry in fs::read_dir(from)? {
    let entry = entry?;
    let entry_type = entry.file_type()?;
    if entry_type.is_dir() {
      copy_dir_all(entry.path(), to.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}
