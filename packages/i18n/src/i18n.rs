use super::{
  config,
  file::{parse, Cache, JsonObject},
};
use napi::{Error, Result, Status};
use napi_common::path::PathExt;
use napi_derive::napi;
use std::path;

lazy_static::lazy_static! {
  static ref CACHE: Cache = Cache::new();
  static ref BRACKETS_RE: regex::Regex = regex::Regex::new(r"\{\{([\w\.]+)\}\}").unwrap();
  static ref LOCALE_RE: regex::Regex = regex::Regex::new(r"[a-z]{2,2}([-_])[A-Z]{2,2}").unwrap();
  static ref LOCALE_STRICT_RE: regex::Regex = regex::Regex::new(r"^([a-z]{2,2}([-_])[A-Z]{2,2})$").unwrap();
  static ref FILENAME_RE: regex::Regex = regex::Regex::new(r"^(.*?)\.[^.]+$").unwrap();
}

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
  pub fn new(options: config::Config) -> Result<Self> {
    let dir = path::absolute(options.directory).map_err(|e| Error::new(Status::InvalidArg, e))?;

    if !dir.exists() || !dir.is_dir() {
      return Err(Error::new(Status::InvalidArg, "Invalid path provided"));
    }

    if let Some(invalid_locale) = options
      .locales
      .iter()
      .chain(options.fallback.iter())
      .chain(options.default.iter())
      .chain(options.locales.iter())
      .find(|locale| !is_locale(locale))
    {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid locale \"{}\"", invalid_locale),
      ));
    }

    let i18n = I18n {
      directory: dir.normalize(),
      locale: options.default.unwrap_or(options.locales[0].clone()),
      fallback: options.fallback.unwrap_or(options.locales[0].clone()),
      locales: options.locales,
    };

    if let Some(preload) = options.preload {
      if preload && CACHE.is_empty() {
        i18n.load(None)?;
      }
    }

    Ok(i18n)
  }

  /// Sets the fallback locale for the current instance.
  /// @param {string} locale
  /// @returns {undefined}
  #[napi]
  pub fn set_fallback(&mut self, locale: String) -> Result<()> {
    if self.fallback != locale {
      if is_locale(&locale) {
        self.fallback = locale;
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
  /// @returns {undefined}
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
    Ok(CACHE.contains_key(&locale))
  }

  /// Reloads translations for the given locale and key.
  /// If a locale is provided, removes the translations for that locale.
  /// If a key is provided, removes the translation for that key in the given locale.
  /// If no locale is provided, clears all translations.
  /// @param {string} [locale]
  /// @param {string} [key]
  /// @returns {undefined}
  #[napi]
  pub fn reload(&self, locale: Option<String>, key: Option<String>) -> Result<()> {
    match (locale.clone(), key) {
      (Some(locale), Some(key)) => {
        let key = format!("{}/{}/{}", self.directory, locale, key);
        CACHE.remove(&key);
        self.load(Some(&locale))?;
      }
      (Some(locale), None) => {
        for item in CACHE.clone().into_iter() {
          if item.0.contains(&locale) {
            CACHE.remove(&item.0);
          }
        }
        self.load(Some(&locale))?; // Reloads the locale
      }
      (None, _) => {
        CACHE.clear();
        self.load(None)?;
      }
    }
    Ok(())
  }

  /// translate function
  /// @param {string} key
  /// @param {Record<string, string | number | boolean>} [args]
  /// @returns {string} translate
  #[napi(ts_args_type = "key: string, args?: Record<string, string | number | boolean>")]
  pub fn t(&self, key: String, args: Option<JsonObject>) -> Result<String> {
    self.translate(self.locale.clone(), key, args)
  }

  /// translate function
  /// @param {string} locale
  /// @param {string} key
  /// @param {Record<string, string | number | boolean>} [args]
  /// @returns {string} translate
  #[napi(ts_args_type = "locale: string, key: string, args?: Record<string, string | number | boolean>")]
  pub fn translate(&self, locale: String, key: String, args: Option<JsonObject>) -> Result<String> {
    if !is_locale(&locale) {
      return Err(Error::new(Status::InvalidArg, "Invalid locale provided"));
    }

    // keys - [] invalid
    // keys is 1 (min: 2) invalid
    // keys[0] is 0 (min: 1 len) invalid
    let keys = key.split(':').collect::<Vec<_>>();
    if keys.len() < 2 || keys[0].is_empty() {
      return Err(Error::new(Status::InvalidArg, "Invalid key provided"));
    }

    let file_path = format!("{}/{}/{}", &self.directory, &locale, keys[0]);
    let Some(translations) = CACHE.get(&file_path) else {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Translation not found for \"{}/{}\"", &locale, keys[0]),
      ));
    };

    let data = if keys[1].contains('.') {
      let fragments: Vec<_> = keys[1].split('.').collect();
      let data = translations.get(fragments[0]);
      fragments
        .iter()
        .skip(1)
        .try_fold(data, |acc, &fragment| acc.map(|data| data.get(fragment)))
        .flatten()
    } else {
      translations.get(keys[1])
    };

    if let Some(data) = data.and_then(|d| d.as_str()) {
      if BRACKETS_RE.is_match(data) {
        if let Some(args) = args {
          let result = BRACKETS_RE.replace_all(data, |caps: &regex::Captures| {
            let key = caps.get(1).unwrap().as_str();
            args
              .get(key)
              .map(|a| a.to_string().replace('"', ""))
              .unwrap_or("??".to_string())
          });
          return Ok(result.to_string());
        }
      }
      return Ok(data.to_string());
    } else if locale != self.fallback {
      return self.translate(self.fallback.clone(), key, args);
    }

    Err(Error::new(
      Status::InvalidArg,
      format!("Missing translation for \"{key}\""),
    ))
  }

  // -- Internal methods --

  fn load_file(&self, file_path: &str, is_absolute: bool) -> Result<()> {
    let Some(caps) = FILENAME_RE.captures(file_path) else {
      return Err(Error::new(
        Status::Unknown,
        format!("Unable to parse filename \"{}\"", file_path),
      ));
    };
    let name = caps.get(if is_absolute { 1 } else { 0 }).unwrap().as_str();
    let table = parse(file_path)?;
    CACHE.entry(name.to_string()).or_insert(table);

    Ok(())
  }

  fn load(&self, load_locale: Option<&str>) -> Result<()> {
    let pattern_path = format!("{}/**/**/*.*", self.directory);
    for entry in glob::glob(&pattern_path).unwrap().filter_map(std::result::Result::ok) {
      if entry.is_file() {
        let full_path = entry.normalize();

        let Some(locale) = LOCALE_RE
          .captures(&full_path)
          .and_then(|c| c.get(0))
          .map(|s| s.as_str())
        else {
          // debug
          continue;
        };

        if let Some(load_locale) = load_locale {
          if locale == load_locale {
            self.load_file(&full_path, true)?;
          }
          continue;
        }

        if self.locales.contains(&locale.to_string()) {
          self.load_file(&full_path, true)?;
        }
      }
    }
    Ok(())
  }
}
