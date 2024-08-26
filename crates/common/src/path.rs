/// A trait that provides additional methods for working with file paths.
pub trait PathExt {
  /// Normalizes the path when on Windows to using forward slash delimiters.
  ///
  /// # Examples
  ///
  /// ```
  /// use common::path::{Path, PathExt};
  ///
  /// let path = Path::new("C:\\Users\\username\\file.txt");
  /// let normalized_path = path.normalize();
  /// assert_eq!(normalized_path, "C:/Users/username/file.txt");
  /// ```
  fn normalize(&self) -> String;

  /// Returns the file extension of the path, if any.
  ///
  /// # Examples
  ///
  /// ```
  /// use common::path::{Path, PathExt};
  ///
  /// let path = Path::new("file.txt");
  /// let extension = path.get_extension();
  /// assert_eq!(extension, "txt");
  ///
  /// let path = Path::new("file");
  /// let extension = path.get_extension();
  /// assert_eq!(extension, "");
  /// ```
  fn get_extension(&self) -> &str;
}

impl PathExt for std::path::Path {
  #[inline]
  fn normalize(&self) -> String {
    self.to_string_lossy().replace("\\", "/").replace("/./", "/")
  }

  #[inline]
  fn get_extension(&self) -> &str {
    self.extension().and_then(std::ffi::OsStr::to_str).unwrap_or_default()
  }
}
