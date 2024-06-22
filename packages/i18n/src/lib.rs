#![deny(clippy::all)]
#![allow(dead_code)]

extern crate allocator;

mod config;
mod file;

use napi::{Error, Result, Status};
use napi_derive::napi;
use once_cell::sync::Lazy;

use std::path;

use self::config::Config;

static BRACKETS_RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"#\{([\w\.]+)\}").unwrap());
static LOCALE_RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"[a-z]{2,2}(\-|\_)[A-Z]{2,2}").unwrap());
static LOCALE_STRICT_RE: Lazy<regex::Regex> =
  Lazy::new(|| regex::Regex::new(r"^[a-z]{2,2}(\-|\_)[A-Z]{2,2}$").unwrap());

#[inline]
fn is_locale(locale: &str) -> bool {
  LOCALE_STRICT_RE.is_match(locale)
}

/// Manages languages and store in cache
#[napi(js_name = "I18n")]
pub struct I18n {
  /// @type {string} fallback use if current locale fail
  /// @readonly
  #[napi(readonly)]
  pub fallback: String,
  /// @type {string} locale is the current language
  /// @readonly
  #[napi(readonly)]
  pub locale: String,
  /// @type {string} directory relative or absolute where locales are located.
  /// @readonly
  #[napi(readonly)]
  pub directory: String,

  /// @type {string[]} locales - A list of available locales, if specified.
  /// @readonly
  #[napi(readonly)]
  pub locales: Vec<String>,
}

#[napi]
impl I18n {
  /// Create a new Languages class from the config provide
  /// @param {I18nConfig} options - Options for class I18n
  #[napi(constructor)]
  pub fn new(options: Config) -> Result<Self> {
    let dir = path::absolute(options.directory).map_err(|e| Error::new(Status::InvalidArg, e))?;

    if !dir.exists() || !dir.is_dir() {
      return Err(Error::new(Status::InvalidArg, "Invalid path provided"));
    }

    if let Some(fallback) = &options.fallback {
      if !is_locale(fallback) {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid fallback locale \"{}\"", fallback),
        ));
      }
    }

    if let Some(default) = &options.default {
      if !is_locale(default) {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid default locale \"{}\"", default),
        ));
      }
    }

    if options.locales.is_empty() {
      return Err(Error::new(Status::InvalidArg, "Array of locales is empty"));
    }

    for locale in options.locales.clone() {
      if !is_locale(&locale) {
        return Err(Error::new(Status::InvalidArg, format!("Invalid locale \"{}\"", locale)));
      }
    }

    let i18n = I18n {
      directory: dir.to_string_lossy().replace('\\', "/"),
      locales: options.locales.clone(),
      locale: options.default.unwrap_or(options.locales[0].clone()),
      fallback: options.fallback.unwrap_or(options.locales[0].clone()),
    };

    if let Some(preload) = options.preload {
      if preload {
        let pattern_path = format!("{}/**/**/*.*", i18n.directory);
        for path in glob::glob(&pattern_path).unwrap().filter_map(std::result::Result::ok) {
          if path.is_file() {
            let full_path = path.to_string_lossy().replace('\\', "/");
            if let Some(caps) = LOCALE_RE.captures(&full_path) {
              let locale = caps.get(0).unwrap().as_str();
              // if locales is empty, ignore and load all
              if !i18n.locales.contains(&locale.to_string()) {
                continue;
              }
              // TODO: implement
              println!("{locale}: {full_path}");
            }
          }
        }
      }
    }

    Ok(i18n)
  }
}
