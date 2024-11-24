#![deny(clippy::all)]
#![allow(dead_code)]

extern crate napi_allocator;

mod config;
mod file;
mod i18n;

use napi::{Error, Result, Status};
use napi_derive::napi;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

// Global methods
static I18N: OnceCell<RwLock<i18n::I18n>> = OnceCell::new();

/// Initializes the i18n instance with the provided configuration.
/// @param {I18nConfig} options
/// @returns {boolean}
#[napi]
pub fn init(options: config::Config) -> Result<bool> {
  I18N.get_or_init(|| RwLock::new(i18n::I18n::new(options).unwrap()));
  Ok(true)
}

// Helper para operaciones de lectura
fn with_i18n_read<F, T>(f: F) -> Result<T>
where
  F: FnOnce(&i18n::I18n) -> Result<T>,
{
  let i18n = I18N
    .get()
    .ok_or_else(|| Error::new(Status::GenericFailure, "Not yet initialized..."))?
    .read();
  f(&i18n)
}

// Helper para operaciones de escritura
fn with_i18n_write<F, T>(f: F) -> Result<T>
where
  F: FnOnce(&mut i18n::I18n) -> Result<T>,
{
  let mut i18n = I18N
    .get()
    .ok_or_else(|| Error::new(Status::GenericFailure, "Not yet initialized..."))?
    .write();
  f(&mut i18n)
}

/// Sets the fallback locale for the current instance.
/// @param {string} locale
/// @returns {undefined}
#[napi]
pub fn set_fallback(locale: String) -> Result<()> {
  with_i18n_write(|i18n| i18n.set_fallback(locale))
}

/// Sets the current locale.
/// @param {string} locale
/// @returns {undefined}
#[napi]
pub fn set_locale(locale: String) -> Result<()> {
  with_i18n_write(|i18n| i18n.set_locale(locale))
}

/// Checks if translations are available for the given locale.
/// Returns true if the locale is present in the translations map, false otherwise.
/// @param {string} locale
/// @returns {boolean} has
#[napi]
pub fn has(locale: String) -> Result<bool> {
  with_i18n_read(|i18n| i18n.has(locale))
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
  with_i18n_write(|i18n| i18n.reload(locale, key))
}

/// translate function
/// @param {string} key
/// @param {Record<string, string | number | boolean>} [args]
/// @returns {string} translate
#[napi(ts_args_type = "key: string, args?: Record<string, string | number | boolean>")]
pub fn t(key: String, args: Option<file::JsonObject>) -> Result<String> {
  with_i18n_read(|i18n| i18n.t(key, args))
}

/// translate function
/// @param {string} locale
/// @param {string} key
/// @param {Record<string, string | number | boolean>} [args]  
/// @returns {string} translate
#[napi(ts_args_type = "locale: string, key: string, args?: Record<string, string | number | boolean>")]
pub fn translate(locale: String, key: String, args: Option<file::JsonObject>) -> Result<String> {
  with_i18n_read(|i18n| i18n.translate(locale, key, args))
}
