#![deny(clippy::all)]
#![allow(dead_code)]

extern crate allocator;

mod config;
mod file;

use napi::{Error, Result, Status};
use napi_derive::napi;
use once_cell::sync::Lazy;

use std::{
  path,
  sync::{Mutex, MutexGuard},
};

use self::{
  config::Config,
  file::{parse, TObject, Translations},
};

static CACHE: Lazy<Mutex<Translations>> = Lazy::new(|| Mutex::new(Translations::new()));
static BRACKETS_RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"#\{([\w\.]+)\}").unwrap());
static LOCALE_RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"[a-z]{2,2}(\-|\_)[A-Z]{2,2}").unwrap());
static LOCALE_STRICT_RE: Lazy<regex::Regex> =
  Lazy::new(|| regex::Regex::new(r"^[a-z]{2,2}(\-|\_)[A-Z]{2,2}$").unwrap());
static FILENAME_RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"^(.*?)\.[^.]+$").unwrap());

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
  ///
  /// Example:
  /// ```js
  /// const i18n = new I18n({
  ///   directory: './locales',
  ///   fallback: 'en-US',
  ///   default: 'fr-FR',
  ///   locales: ['fr-FR', 'en-US', 'es-ES'],
  ///   preload: true,
  /// });
  /// ```
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

    let mut i18n = I18n {
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

            let Some(locale) = LOCALE_RE
              .captures(&full_path)
              .and_then(|c| c.get(0))
              .map(|s| s.as_str())
            else {
              // debug
              continue;
            };
            if i18n.locales.contains(&locale.to_string()) {
              _ = i18n.get_with_path(locale, &full_path)?;
            }
          }
        }
      }
    }
    Ok(i18n)
  }

  /// Sets the fallback locale for the current instance.
  /// @param {string} locale
  /// @returns {void}
  #[napi]
  pub fn set_fallback(&mut self, locale: String) -> Result<()> {
    if self.fallback != locale {
      if is_locale(&locale) {
        self.fallback = locale
      } else {
        return Err(Error::new(
          Status::InvalidArg,
          "Invalid locale provided, eg: en-US, es-ES, pr-BR...",
        ));
      }
    }

    Ok(())
  }

  /// Sets the current locale.
  /// @param {string} locale
  /// @returns {void}
  #[napi]
  pub fn set_locale(&mut self, locale: String) -> Result<()> {
    if self.locale != locale {
      if is_locale(&locale) {
        self.locale = locale;
      } else {
        return Err(Error::new(Status::InvalidArg, "Invalid locale provided"));
      }
    }

    Ok(())
  }

  /// Checks if translations are available for the given locale.
  /// Returns true if the locale is present in the translations map, false otherwise.
  /// @param {string} locale
  /// @returns {boolean} has
  #[napi]
  pub fn has(&self, locale: String) -> Result<bool> {
    let c = self.cache()?;
    Ok(c.contains_key(&locale))
  }

  /// Reloads translations for the given locale and key.
  /// If a locale is provided, removes the translations for that locale.
  /// If a key is provided, removes the translation for that key in the given locale.
  /// If no locale is provided, clears all translations.
  /// @param {string} [locale]
  /// @param {string} [key]
  /// @returns {void}
  #[napi]
  pub fn reload(&mut self, locale: Option<String>, key: Option<String>) -> Result<()> {
    let mut cache = self.cache()?;
    match (locale, key) {
      (Some(locale), Some(key)) => {
        let key = format!("{}/{}/{}", self.directory, locale, key);
        cache.entry(locale).and_modify(|e| {
          e.remove(&key);
        });
      }
      (Some(locale), None) => {
        cache.remove(&locale);
      }
      (None, _) => {
        cache.clear();
      }
    }

    Ok(())
  }

  /// -- Internal methods --

  #[inline]
  fn cache(&self) -> Result<MutexGuard<Translations>> {
    CACHE.lock().map_err(|err| {
      Error::new(
        Status::Unknown,
        format!("Unable to access to global cache: \"{}\"", err),
      )
    })
  }

  #[inline]
  fn get(&mut self, locale: &str, file: &str) -> Result<TObject> {
    let file_path = format!("{}/{}/{}", self.directory, locale, file);
    self.get_with_path(locale, &file_path)
  }

  fn get_with_path(&mut self, locale: &str, file_path: &str) -> Result<TObject> {
    let mut cache = self.cache()?;
    if let Some(cache_obj) = cache.get(locale).and_then(|t| t.get(file_path)) {
      return Ok(cache_obj.clone());
    }

    let Some(caps) = FILENAME_RE.captures(file_path) else {
      return Err(Error::new(
        Status::Unknown,
        format!("Unable to parse filename \"{}\"", file_path),
      ));
    };
    let name = caps.get(1).unwrap().as_str();

    let table = parse(file_path)?;
    let _ = cache
      .entry(locale.to_string())
      .or_default()
      .insert(name.to_string(), table.clone());

    Ok(table)
  }
}
