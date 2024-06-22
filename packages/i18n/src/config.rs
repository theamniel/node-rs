use napi_derive::napi;
use serde::Deserialize;

/// A configuration interface for Languages
#[napi(object, js_name = "I18nConfig")]
#[derive(Deserialize, Debug)]
pub struct Config {
  /// directory relative or absolute where locales are located.
  /// @type {string} path
  pub directory: String,

  /// A list of available locales.
  /// @type {string[]} locales
  pub locales: Vec<String>,

  /// fallback language.
  /// @type {string} fallback
  pub fallback: Option<String>,

  /// default language, defaults to the fallback language if not specified.
  /// @type {string} [default]
  pub default: Option<String>,

  /// A flag indicating whether to preload locales on initialization.
  /// @type {boolean} [preload]
  pub preload: Option<bool>,
}
