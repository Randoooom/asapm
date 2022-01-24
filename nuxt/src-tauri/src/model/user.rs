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

use thiserror::Error;
use std::fs;
use std::path::PathBuf;
use pbkdf2::password_hash::{PasswordHash, PasswordVerifier};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use crate::model::backup::Backup;
use crate::model::encryption::Encryption;

#[derive(Deserialize, Serialize)]
pub struct UserData {
  username: String,
  password: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
  #[error(transparent)]
  IOError(#[from] std::io::Error),

  #[error(transparent)]
  ParseError(#[from] serde_json::Error),

  #[error("Unauthorized")]
  Unknown,
}

pub struct User {
  // identification for login
  username: String,
  // only optional for the user
  backup: Option<Backup>,
  // will also save the user password
  pub encryption: Encryption,
}

#[derive(Deserialize, Serialize)]
pub struct RawUser {
  username: String,
  backup: Option<Backup>,
  // hashed
  password: String,
}

impl RawUser {
  /// create an new user just by the username and the disk
  /// does not not contain any usable sensitive data
  fn new_from_disk(directory: &PathBuf, username: &str) -> Result<Self, ConfigError> {
    // create path for the possible user
    let path = directory.join(format!("/{}", username));
    // read file content
    let content = fs::read_to_string(path)?;

    // parse json
    let parsed = serde_json::from_str::<Self>(content.as_str())?;
    Ok(parsed)
  }
}

impl User {
  /// init new full user based on login credentials
  pub fn new_from_login(directory: &PathBuf, data: UserData) -> Result<Self, ConfigError> {
    // load raw user
    let mut raw = RawUser::new_from_disk(directory, data.username.as_str())?;

    // compare passwords
    let hash = PasswordHash::new(raw.password.as_str()).unwrap();
    match Pbkdf2.verify_password(data.password.as_bytes(), &hash) {
      Ok(()) => {
        // init encryption
        let encryption = Encryption::new_from_passphrase(data.password);
        // init backup
        if let Some(backup) = raw.backup {
          raw.backup = Some(backup.init_from_login(&encryption).unwrap());
        }

        Ok(
          Self {
            username: raw.username,
            backup: raw.backup,
            encryption,
          }
        )
      }
      Err(_) => Err(ConfigError::Unknown)
    }
  }

  pub fn backup(&self) -> Option<Backup> {
    self.backup.clone()
  }
}
