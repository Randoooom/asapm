/*
 * MIT LICENSE
 *
 * Copyright (c) 2022 Randoooom
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::iter;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use aes_gcm_siv::aead::{Aead, NewAead};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(Error, Debug)]
pub enum EncryptionError {
  #[error(transparent)]
  UTFError(#[from] std::string::FromUtf8Error),

  #[error(transparent)]
  AESError(#[from] aes_gcm_siv::aead::Error),
}

type Result<T> = std::result::Result<T, EncryptionError>;

/// struct for general encryption and decryption of the data
#[derive(Clone)]
pub struct Encryption {
  cipher: Aes256GcmSiv,
}

#[derive(Deserialize, Serialize)]
pub struct CipherText {
  pub ciphertext: String,
  pub nonce: String,
}

impl Encryption {
  /// generate random nonce
  pub fn generate(length: usize) -> String {
    // generate random bytes
    let bytes = (0..length).map(|_| { rand::random::<u8>() }).collect::<Vec<u8>>();

    base64::encode(bytes)
  }

  /// create new from bytes
  pub fn new(key: &[u8]) -> Self {
    let key = Key::from_slice(key);
    // build cipher
    let cipher = Aes256GcmSiv::new(key);

    Self {
      cipher
    }
  }

  /// decrypt a &str into a String
  pub fn decrypt(&self, data: String, iv: String) -> Result<String> {
    // parse nonce and ciphertext from data
    // create nonce as array
    let nonce = Nonce::from_slice(iv.as_bytes());
    // decrypt
    let plaintext = self.cipher.decrypt(&nonce, base64::decode(data).unwrap().as_slice())?;
    let parsed = String::from_utf8(plaintext.clone()).unwrap_or(base64::encode(plaintext));
    Ok(parsed)
  }

  /// encrypt data with random nonce
  pub fn encrypt(&self, data: &str) -> Result<CipherText> {
    // generate nonce
    let nonce_string = Self::generate(8);
    let nonce = Nonce::from_slice(nonce_string.as_bytes());

    // encrypt
    let ciphertext = self.cipher.encrypt(&nonce, data.as_bytes())?;
    let ciphertext = base64::encode(ciphertext);

    // return nonce and ciphertext
    Ok(
      CipherText {
        ciphertext,
        nonce: nonce_string,
      }
    )
  }
}
