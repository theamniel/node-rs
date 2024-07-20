extern crate global_allocator;

use aes::{
  cipher::{generic_array, KeyIvInit, StreamCipher},
  Aes256,
};
use ctr::Ctr128BE;
use napi::{bindgen_prelude::Buffer, Error, Result, Status};
use napi_derive::napi;

type Key = generic_array::GenericArray<u8, generic_array::typenum::U32>;
type Nonce = generic_array::GenericArray<u8, generic_array::typenum::U16>;

/**
 * Encrypt a given text using the provided secret key and initialization vector (IV).
 *
 * @param {string} text
 * @param {Buffer} secret
 * @param {Buffer} iv
 * @returns {string} Data encrypted in hash
 */
#[napi]
pub fn encrypt(text: String, secret: Buffer, iv: Buffer) -> Result<String> {
  let (key, nonce) = get_key_and_nonce(&secret, &iv)?;

  let mut cipher = Ctr128BE::<Aes256>::new(&key, &nonce);
  let mut encrypted = text.into_bytes();

  cipher
    .try_apply_keystream(&mut encrypted)
    .map_err(|e| Error::new(Status::Unknown, e))?;
  Ok(hex::encode(encrypted))
}

/**
 * Decrypt a given ciphertext using the provided secret key and initialization vector (IV).
 *
 * @param {string} ciphertext
 * @param {Buffer} secret
 * @param {Buffer} iv
 * @returns {string} Data decrypted into a String
 */
#[napi]
pub fn decrypt(ciphertext: String, secret: Buffer, iv: Buffer) -> Result<String> {
  let (key, nonce) = get_key_and_nonce(&secret, &iv)?;

  let mut decrypted = hex::decode(ciphertext).map_err(|e| Error::new(Status::GenericFailure, e))?;
  let mut cipher = Ctr128BE::<Aes256>::new(&key, &nonce);

  cipher
    .try_apply_keystream(&mut decrypted)
    .map_err(|e| Error::new(Status::Unknown, e))?;

  String::from_utf8(decrypted).map_err(|e| Error::new(Status::GenericFailure, e))
}

/**
 * Cycle a given Number within a specified range, optionally in reverse.
 *
 * @param {number} num
 * @param {number} count
 * @param {boolean} [negative=false]
 * @returns {number} reverse Number
 */
#[napi]
pub fn cycle(mut num: i32, count: i32, negative: Option<bool>) -> i32 {
  let (min, max) = (0, 9);
  let increment = if negative.unwrap_or(false) { -1 } else { 1 };

  for _ in 0..count {
    num = (num + increment + max - min + 1) % (max - min + 1) + min;
  }

  num
}

/// Extract the key and nonce from the provided secret and IV
#[inline]
fn get_key_and_nonce(secret: &Buffer, iv: &Buffer) -> Result<(Key, Nonce)> {
  if secret.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      "Invalid secret key length. Must be 32 bytes.",
    ));
  }

  if iv.len() != 16 {
    return Err(Error::new(
      Status::InvalidArg,
      "Invalid inicialization vector (iv) length. Must be 16 bytes.",
    ));
  }

  let key = Key::from_slice(secret);
  let nonce = Nonce::from_slice(iv);

  Ok((*key, *nonce))
}
