use common::path::PathExt;
use std::path::Path;

#[test]
fn test_normalize_windows_path() {
  let path = Path::new("C:\\Users\\username\\file.txt");
  let normalized_path = path.normalize();
  assert_eq!(normalized_path, "C:/Users/username/file.txt");
}

#[test]
fn test_normalize_unix_path() {
  let path = Path::new("/Users/username/file.txt");
  let normalized_path = path.normalize();
  assert_eq!(normalized_path, "/Users/username/file.txt");
}

#[test]
fn test_normalize_path_with_dot_unix() {
  let path = Path::new("/./Users/username/./file.txt");
  let normalized_path = path.normalize();
  assert_eq!(normalized_path, "/Users/username/file.txt");
}

#[test]
fn test_get_extension() {
  let path = Path::new("file.txt");
  let extension = path.get_extension();
  assert_eq!(extension, "txt");
}

#[test]
fn test_get_extension_no_extension() {
  let path = Path::new("file");
  let extension = path.get_extension();
  assert_eq!(extension, "");
}

#[test]
fn test_get_extension_multiple_dots() {
  let path = Path::new("file.txt.gz");
  let extension = path.get_extension();
  assert_eq!(extension, "gz");
}
