#![deny(clippy::all)]
#![allow(dead_code)]

extern crate global_allocator;

mod config;
mod file;
mod i18n;

use napi::{Error, Result, Status};
use napi_derive::napi;
use std::sync::{OnceLock, RwLock};

// Global methods
static I18N: OnceLock<RwLock<i18n::I18n>> = OnceLock::new();

/**
 * @param {I18nConfig} options
 * @returns {boolean}
 */
#[napi]
pub fn init(options: config::Config) -> Result<bool> {
  if I18N.get().is_none() {
    let i18n_instance = i18n::I18n::new(options)?;
    _ = I18N.set(RwLock::new(i18n_instance));
    return Ok(true);
  }
  Ok(false)
}

/// Sets the fallback locale for the current instance.
/// @param {string} locale
/// @returns {undefined}
#[napi]
pub fn set_fallback(locale: String) -> Result<()> {
  if let Some(i18n) = I18N.get() {
    return i18n.write().unwrap().set_fallback(locale);
  }

  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}

/// Sets the current locale.
/// @param {string} locale
/// @returns {undefined}
#[napi]
pub fn set_locale(locale: String) -> Result<()> {
  if let Some(i18n) = I18N.get() {
    return i18n.write().unwrap().set_locale(locale);
  }
  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}

/// Checks if translations are available for the given locale.
/// Returns true if the locale is present in the translations map, false otherwise.
/// @param {string} locale
/// @returns {boolean} has
#[napi]
pub fn has(locale: String) -> Result<bool> {
  if let Some(i18n) = I18N.get() {
    return i18n.read().unwrap().has(locale);
  }
  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}

/// Reloads translations for the given locale and key.
/// If a locale is provided, removes the translations for that locale.
/// If a key is provided, removes the translation for that key in the given locale.
/// If no locale is provided, clears all translations.
/// @param {string} [locale]
/// @param {string} [key]
/// @returns {undefined}
#[napi]
pub fn reload(locale: Option<String>, key: Option<String>) -> Result<()> {
  if let Some(i18n) = I18N.get() {
    return i18n.read().unwrap().reload(locale, key);
  }
  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}

/// translate function
/// @param {string} key
/// @param {Record<string, string | number | boolean>} [args]
/// @returns {string} translate
#[napi(ts_args_type = "key: string, args?: Record<string, string | number | boolean>")]
pub fn t(key: String, args: Option<file::JsonObject>) -> Result<String> {
  if let Some(i18n) = I18N.get() {
    return i18n.read().unwrap().t(key, args);
  }
  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}

/// translate function
/// @param {string} locale
/// @param {string} key
/// @param {Record<string, string | number | boolean>} [args]  
/// @returns {string} translate
#[napi(ts_args_type = "locale: string, key: string, args?: Record<string, string | number | boolean>")]
pub fn translate(locale: String, key: String, args: Option<file::JsonObject>) -> Result<String> {
  if let Some(i18n) = I18N.get() {
    return i18n.read().unwrap().translate(locale, key, args);
  }
  Err(Error::new(Status::GenericFailure, "Not yet initialized..."))
}
