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
use rand::Rng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::backup::Backup;
use crate::model::encryption::{Encryption, EncryptionError};

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

  #[error(transparent)]
  EncryptionError(#[from] EncryptionError),

  #[error("Unauthorized")]
  Unknown,
}

#[derive(Deserialize, Serialize, Clone)]
// complete serde::to_string() and aes encrypted on the disk (base64)
pub struct PasswordData {
  login: Option<String>,
  password: Option<String>,
  url: Option<String>,
  description: Option<String>,
  // identification
  uuid: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Password {
  // will change on each write (properly)
  iv: String,
  // encrypted, would be PasswordDat
  data: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum PasswordType {
  Data(PasswordData),
  Raw(Password),
}

#[derive(Serialize, Deserialize)]
pub struct User {
  // identification for login
  username: String,
  // only optional for the user
  backup: Option<Backup>,
  // will also save the user password
  #[serde(skip_serializing, skip_deserializing)]
  pub encryption: Option<Encryption>,
  // the password data
  password: RawUserPassword,
  // must be decrypted and decoded
  passwords: Vec<PasswordType>,
}

#[derive(Deserialize, Serialize, Clone)]
struct RawUserPassword {
  // aes encrypted random key
  key: String,
  // the key iv
  iv: String,
  // the hash of the hash
  hash: String,
  // the original salt
  salt: String,
}

#[derive(Deserialize, Serialize)]
pub struct RawUser {
  username: String,
  backup: Option<Backup>,
  password: RawUserPassword,
  // base64 encoded and encrypted
  passwords: Vec<Password>,
}

impl RawUser {
  /// create an new user just by the username and the disk
  /// does not not contain any usable sensitive data
  fn new_from_disk(directory: &PathBuf, username: &str) -> Result<Self, ConfigError> {
    // create path for the possible user
    let path = directory.join(username);
    // read file content
    let content = fs::read_to_string(path)?;

    // parse json
    let parsed = serde_json::from_str::<Self>(content.as_str())?;
    Ok(parsed)
  }
}

impl User {
  /// create new user from signup information
  pub fn new_from_signup(directory: &PathBuf, data: UserData) -> Result<Self, ConfigError> {
    // create the path
    let path = directory.join(&data.username);
    // check for already existing user
    match &path.exists() {
      // return err on true, because we will not overwrite any userdata
      true => return Err(ConfigError::Unknown),
      false => {
        // hash the password
        let salt = SaltString::generate(&mut OsRng);
        let hash_salt = salt.as_str();
        let hash = Pbkdf2.hash_password(data.password.as_bytes(), &salt).unwrap();

        // setup the file key as random base64
        let key = Encryption::generate(32);
        // build the initial encryption
        let encryption = Encryption::new(base64::decode(&key).unwrap().as_slice());

        // encrypt it for storage
        let key_encryption = Encryption::new(hash.hash.unwrap().as_bytes());
        let key = key_encryption.encrypt(key.as_str())?;

        // second time because we encrypt the key with the first hash
        let salt = SaltString::generate(&mut OsRng);
        let hash = Pbkdf2.hash_password(hash.to_string().as_bytes(), &salt).unwrap();

        // init the user
        let mut user = Self {
          username: data.username,
          encryption: Some(encryption.clone()),
          backup: None,
          password: RawUserPassword {
            key: key.ciphertext,
            iv: key.nonce,
            hash: hash.to_string(),
            salt: hash_salt.to_string(),
          },
          passwords: Vec::new(),
        };

        // save the data
        Self::write_file(&path, &mut user)?;

        // return ok
        Ok(user)
      }
    }
  }

  /// init new full user based on login credentials
  pub fn new_from_login(directory: &PathBuf, data: UserData) -> Result<Self, ConfigError> {
    // load raw user
    let mut raw = RawUser::new_from_disk(directory, data.username.as_str())?;
    let cloned = raw.password.clone();

    // compare passwords
    let hash = PasswordHash::new(cloned.hash.as_str()).unwrap();
    // hash the input password
    let salt = SaltString::new(raw.password.salt.as_str()).unwrap();
    let password = Pbkdf2.hash_password(data.password.as_bytes(), &salt).unwrap();
    // match the hashes
    match Pbkdf2.verify_password(password.to_string().as_bytes(), &hash) {
      Ok(()) => {
        // decrypt the stored key
        let encryption = Encryption::new(password.hash.unwrap().as_bytes());
        let key = encryption.decrypt(cloned.key, cloned.iv)?;

        // create new encryption for the user
        let encryption = Encryption::new(base64::decode(key).unwrap().as_slice());

        // init backup
        if let Some(backup) = raw.backup {
          raw.backup = Some(backup.init_from_login(&encryption).unwrap());
        }

        // decrypt the passwords
        let passwords = raw.passwords.iter().map(|password| {
          // decrypt the data
          let raw = encryption.decrypt(password.data.clone(), password.iv.clone()).unwrap();
          // parse json
          PasswordType::Data(serde_json::from_str::<PasswordData>(raw.as_str()).unwrap())
        })
          .collect::<Vec<PasswordType>>();

        Ok(
          Self {
            username: raw.username,
            backup: raw.backup,
            encryption: Some(encryption),
            password: raw.password,
            passwords,
          }
        )
      }
      Err(_) => Err(ConfigError::Unknown)
    }
  }

  /// write the userdata into the file
  pub fn write_file(path: &PathBuf, data: &mut Self) -> Result<(), ConfigError> {
    data.passwords = data.passwords.iter().map(|password| {
      // stringify
      let raw = serde_json::to_string(password).unwrap();
      // encrypt
      let encrypted = data.encryption.clone().unwrap().encrypt(raw.as_str()).unwrap();

      PasswordType::Raw(
        Password {
          iv: encrypted.nonce,
          data: encrypted.ciphertext,
        }
      )
    }).collect::<Vec<PasswordType>>();

    // convert to string
    let raw = serde_json::to_string(&data).unwrap();
    // write the bytes from the raw data into the file
    fs::write(path, raw)?;

    Ok(())
  }

  /// create new password
  pub fn new_password(&mut self) -> PasswordData {
    let data = PasswordData {
      login: None,
      password: None,
      url: None,
      description: None,
      uuid: Uuid::new_v4().to_string(),
    };

    // push the new password
    self.passwords.push(PasswordType::Data(data.clone()));
    data
  }

  /// update an specific password
  pub fn update_password(&mut self, data: PasswordData) {
    // update an existing password
    self.passwords = self.passwords.clone().into_iter().map(|ty| {
      if let PasswordType::Data(password) = ty.clone() {
        if password.clone().uuid.eq(&data.uuid) {
          return PasswordType::Data(data.clone());
        }
      }
      ty
    }).collect::<Vec<PasswordType>>();
  }

  /// delete an existing password
  pub fn delete_password(&mut self, data: PasswordData) {
    // remove it via filter
    self.passwords = self.passwords.clone().into_iter().filter(|ty| {
      if let PasswordType::Data(password) = ty.clone() {
        return !password.uuid.eq(&data.uuid);
      }
      true
    }).collect::<Vec<PasswordType>>();
  }

  pub fn backup(&self) -> Option<Backup> {
    self.backup.clone()
  }

  pub fn passwords(&self) -> Vec<PasswordType> {
    self.passwords.clone()
  }
}
