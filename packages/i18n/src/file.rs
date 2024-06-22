use napi::{Error, Result, Status};
use std::{collections::HashMap, fs, path::PathBuf};

pub type TObject = HashMap<String, serde_json::Value>;
pub type Translations = HashMap<String, HashMap<String, TObject>>;

#[inline]
pub fn resolve_path(file: &str) -> Result<PathBuf> {
  let path = PathBuf::from(file);
  if !path.exists() || !path.is_file() {
    return Err(Error::new(
      Status::InvalidArg,
      format!("File not found \"{}\"", path.display()),
    ));
  }
  Ok(path)
}

#[inline]
pub fn parse(full_path: &str) -> Result<TObject> {
  let path = resolve_path(full_path)?;
  let content = fs::read_to_string(&path).map_err(|_| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to read file \"{}\"", path.display()),
    )
  })?;

  parse_content(&content, path.extension().unwrap().to_string_lossy().as_ref())
}

#[inline]
pub fn parse_content(content: &str, ext: &str) -> Result<TObject> {
  match ext {
    "json" => Ok(serde_json::from_str::<TObject>(content)?),
    "toml" => toml::from_str::<TObject>(content).map_err(|e| Error::new(Status::GenericFailure, e)),
    "yaml" | "yml" => serde_yml::from_str::<TObject>(content).map_err(|e| Error::new(Status::GenericFailure, e)),
    _ => Err(Error::new(Status::InvalidArg, "Invalid file extension")),
  }
}
