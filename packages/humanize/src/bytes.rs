/// This is a fork and modify version of [humanize_bytes](https://github.com/trueb2/humanize-bytes)
///
/// The MIT License (MIT)
/// Copyright (c) 2022 Jacob Trueb
///
/// The Apache License (Apache License, Version 2.0)
/// Copyright (c) 2022 Jacob Trueb
use napi_derive::napi;

const UNIT: f64 = 1024.0;
const SUFFIX: [&str; 11] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB"];

/**
 * Humanize a byte size into a string.
 *
 * @param {number} bytes
 * @returns {string} a string representation of that size in a human-readable format.
 */
#[napi]
pub fn bytes(bytes: f64) -> String {
  let (value, unit) = {
    let base = if bytes < 0.0 { -bytes } else { bytes }.log2() as usize / 10;
    let pow_base = UNIT.powi(base as i32);
    let units = ((bytes / pow_base) * 100.0).floor() / 100.0;

    (units, SUFFIX[base])
  };

  format!("{} {}", value, unit)
}
