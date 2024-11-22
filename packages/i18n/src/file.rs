use napi_common::path::PathExt;
use dashmap::DashMap;
use napi::{Error, Result, Status};
use std::{collections::HashMap, fs, path::PathBuf};

/// A type alias for JSON object represented as a HashMap of String to serde_json::Value.
pub type JsonObject = HashMap<String, serde_json::Value>;

/// A type alias for a translation object represented as a DashMap of String to JsonObject.
pub type Cache = DashMap<String, JsonObject>;

/// A type alias for a translation file.
const EXTS: [&str; 4] = ["json", "toml", "yaml", "yml"];

/// Resolves a file path to a PathBuf.
///
/// # Errors
///
/// Returns an Error if the file does not exist or is not a file.
pub fn resolve_path(file: &str) -> Result<PathBuf> {
  let mut path = PathBuf::from(file);
  if path.exists() && path.is_file() {
    return Ok(path);
  }

  for ext in EXTS {
    path.set_extension(ext);
    if path.exists() && path.is_file() {
      return Ok(path);
    }
    path.set_extension(""); // reset to prevent
  }
  Err(Error::new(Status::InvalidArg, format!("File not found \"{file}\"")))
}

/// Parses a file at the given path into a TObject.
///
/// # Errors
///
/// Returns an Error if the file does not exist, is not a file or cannot be parsed.
#[inline]
pub fn parse(full_path: &str) -> Result<JsonObject> {
  let path = resolve_path(full_path)?;
  let content = fs::read_to_string(&path).map_err(|_| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to read file \"{}\"", path.display()),
    )
  })?;

  parse_content(&content, path.get_extension())
}

/// Parses a string content into a TObject based on the file extension.
///
/// # Errors
/// Returns an Error if the content cannot be parsed of the file extension is invalid.
#[inline]
pub fn parse_content<T: serde::de::DeserializeOwned>(content: &str, ext: &str) -> Result<T> {
  match ext {
    "json" => Ok(serde_json::from_str::<T>(content)?),
    "toml" => toml::from_str::<T>(content).map_err(|e| Error::new(Status::GenericFailure, e)),
    "yaml" | "yml" => serde_yml::from_str::<T>(content).map_err(|e| Error::new(Status::GenericFailure, e)),
    _ => Err(Error::new(Status::InvalidArg, "Invalid file extension")),
  }
}
