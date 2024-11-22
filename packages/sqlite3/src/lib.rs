#![deny(clippy::all)]

extern crate napi_allocator;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn floor(n: f64) -> f64 {
  n.floor()
}
