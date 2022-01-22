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
pub struct Encryption {
  cipher: Aes256GcmSiv,
}

pub struct CipherText {
  ciphertext: String,
  nonce: String,
}

impl Encryption {
  /// generate random nonce
  pub fn generate_nonce() -> String {
    // init rng
    let mut rng = thread_rng();

    // generate
    iter::repeat(())
      .map(|()| rng.sample(Alphanumeric))
      .map(char::from)
      .take(12)
      .collect::<String>()
  }

  /// create new from passphrase
  pub fn new_from_passphrase(passphrase: String) -> Self {
    // parse to bytes
    let key = Key::from_slice(passphrase.as_bytes());
    // build cipher
    let cipher = Aes256GcmSiv::new(key);

    Self {
      cipher
    }
  }

  /// decrypt a &str into a String
  pub fn decrypt(&self, data: &str) -> Result<String> {
    // parse nonce and ciphertext from data
    let nonce = &data[..12].to_string();
    let ciphertext = &data[13..].to_string();
    // create nonce as array
    let nonce = Nonce::from_slice(nonce.as_bytes());

    // decrypt
    let plaintext = self.cipher.decrypt(&nonce, ciphertext.as_bytes())?;
    let parsed = String::from_utf8(plaintext)?;
    Ok(parsed)
  }

  /// encrypt data with random nonce
  pub fn encrypt(&self, data: &str) -> Result<CipherText> {
    // generate nonce
    let nonce_string = Self::generate_nonce();
    let nonce = Nonce::from_slice(nonce_string.as_bytes());

    // encrypt
    let ciphertext = self.cipher.encrypt(&nonce, data.as_bytes())?;
    let ciphertext = String::from_utf8(ciphertext)?;

    // return nonce and ciphertext
    Ok(
      CipherText {
        ciphertext,
        nonce: nonce_string,
      }
    )
  }
}
