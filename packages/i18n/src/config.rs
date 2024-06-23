use napi_derive::napi;
use serde::Deserialize;

/// Represents the configuration for internationalization (i18n).
#[napi(object, js_name = "I18nConfig")]
#[derive(Deserialize, Debug)]
pub struct Config {
  /// The directory where locale files are stored.
  /// @type {string} directory
  pub directory: String,

  /// A list of supported locales.
  /// @type {string[]} locales
  pub locales: Vec<String>,

  /// The fallback locale to use when a translation is not found.
  /// @type {string} [fallback]
  pub fallback: Option<String>,

  /// The default locale to use when no locale is specified.
  /// @type {string} [default]
  pub default: Option<String>,

  /// Whether to preload all locale files or not.
  /// @type {boolean} [preload]
  pub preload: Option<bool>,
}
