use napi_derive::napi;

const M_SECOND: f64 = 60.0;
const M_MINUTE: f64 = M_SECOND;
const M_HOUR: f64 = 24.0;
const M_DAY: f64 = 30.42;
const M_WEEK: f64 = 4.0;
const M_MONTH: f64 = 12.0;
const M_YEAR: f64 = 0.0;

const SECOND: f64 = 1.0;
const MINUTE: f64 = 60.0 * SECOND;
const HOUR: f64 = 60.0 * MINUTE;
const DAY: f64 = 24.0 * HOUR;
const WEEK: f64 = 7.0 * DAY;
const MONTH: f64 = 30.42 * DAY;
const YEAR: f64 = 365.25 * DAY;

// TODO
// const YEARLEAP: f64 = YEAR + (18.0 * HOUR);

// #[inline]
// const fn is_leap_year(year: i32) -> bool {
//   year % 4 == 0 && year % 100 != 0 && year % 400 != 0
// }

/// UNITS is a constant array of tuples containing the divisor, modulus, singular, plural, and abbreviation for each unit
const UNITS: [(f64, f64, &str, &str, &str); 7] = [
  (YEAR, M_YEAR, "year", "years", "y"),
  (MONTH, M_MONTH, "month", "months", "m"),
  (WEEK, M_WEEK, "week", "weeks", "w"),
  (DAY, M_DAY, "day", "days", "d"),
  (HOUR, M_HOUR, "hour", "hours", "h"),
  (MINUTE, M_MINUTE, "minute", "minutes", "min"),
  (SECOND, M_SECOND, "second", "seconds", "sec"),
];

/// Assuming milliseconds shouldn't exceed seconds represented by `f64::MAX`
const MAX_MS: f64 = f64::MAX / 1000.0;

/**
 * Humanize a duration in milliseconds to a string.
 *
 * @param {number} ms - the duration in milliseconds.
 * @param {number} [maxUnits=7] - the maximum number of units to display (default is 7).
 * @param {boolean} [short=false] - Whether to use short abbreviations (default is false)
 * @returns {string} a human-readable string representation of the duration.
 */
#[napi]
pub fn duration(ms: f64, max_units: Option<i32>, short: Option<bool>) -> String {
  if !(1.0..=MAX_MS).contains(&ms) {
    return "0".to_string();
  }

  let is_short = short.unwrap_or(false);
  let max = max_units.unwrap_or(7).clamp(1, 7) as usize;
  let mut units = Vec::with_capacity(max);

  generate_parsers(&ms, max, |(value, singular, plural, abbrev)| {
    units.push(if is_short {
      format!("{value:.0}{abbrev}")
    } else if value > 1.0 {
      format!("{value:.0} {plural}")
    } else {
      format!("{value:.0} {singular}")
    });
  });

  units
    .iter()
    .enumerate()
    .map(|(i, res)| {
      if is_short || max == 1 {
        res.to_string()
      } else {
        match units.len() {
          len if len >= 2 && i == len - 2 => format!("{res} and"),
          len if len > 1 && i != len - 1 => format!("{res},"),
          _ => res.to_string(),
        }
      }
    })
    .collect::<Vec<String>>()
    .join(" ")
}

/**
 * Generate parsers for the given duration.
 *
 * # Parameters
 *
 * * `ms`: The duration in milliseconds.
 * * `max_units`: The maximum number of units to display.
 * * `add`: A closure to add the parsed units to a vector.
 */
#[inline]
fn generate_parsers<F: FnMut((f64, &str, &str, &str))>(ms: &f64, max_units: usize, add: F) {
  UNITS
    .into_iter()
    .filter_map(|(d, m, s, p, a)| {
      let value = if m == M_YEAR { round(ms / d) } else { round(ms / d) % m };
      if value > 0.0 {
        Some((value, s, p, a))
      } else {
        None
      }
    })
    .take(max_units)
    .for_each(add);
}

#[inline]
fn round(ms: f64) -> f64 {
  ms.signum() * ms.abs().floor()
}
