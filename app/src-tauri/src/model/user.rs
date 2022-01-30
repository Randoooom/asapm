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
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::backup::Backup;
use crate::model::encryption::{Encryption, EncryptionError};
use crate::model::generator::PasswordGenerator;

#[derive(Deserialize, Serialize)]
pub struct AnalyseResult {
  // vec of the matching uuids
  reused: Vec<String>,
  very_strong: Vec<String>,
  strong: Vec<String>,
  medium: Vec<String>,
  weak: Vec<String>,
  very_weak: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
// complete serde::to_string() and aes encrypted on the disk (base64)
pub struct PasswordData {
  name: Option<String>,
  login: Option<String>,
  password: Option<String>,
  url: Option<String>,
  description: Option<String>,
  // identification
  uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Password {
  // will change on each write (properly)
  iv: String,
  // encrypted, would be PasswordDat
  data: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
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
  // the default generator for the user
  generator: PasswordGenerator,
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
  generator: PasswordGenerator,
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

  fn write_to_disk(&self, path: &PathBuf) -> Result<(), ConfigError> {
    // stringify
    let raw = serde_json::to_string(&self).unwrap();
    // write the bytes into the file
    fs::write(&path, raw.as_bytes())?;
    Ok(())
  }
}

impl From<&User> for RawUser {
  fn from(user: &User) -> Self {
    // encrypt the passwords
    let passwords = user.passwords.iter().map(|password| {
      // stringify
      let raw = serde_json::to_string(password).unwrap();
      // encrypt
      let encrypted = user.encryption.clone().unwrap().encrypt(raw.as_str()).unwrap();

      Password {
        iv: encrypted.nonce,
        data: encrypted.ciphertext,
      }
    }).collect::<Vec<Password>>();

    Self {
      username: user.username(),
      backup: user.backup(),
      password: user.password.clone(),
      passwords,
      generator: user.generator(),
    }
  }
}

impl User {
  /// get the username
  pub fn username(&self) -> String {
    self.username.clone()
  }

  /// get the generator
  pub fn generator(&self) -> PasswordGenerator {
    self.generator.clone()
  }

  /// update the default generator
  pub fn update_generator(&mut self, generator: PasswordGenerator) {
    self.generator = generator
  }

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
        let user = Self {
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
          generator: PasswordGenerator::default(),
        };

        // save the data
        Self::write(&user, &path.parent().unwrap().to_path_buf())?;

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
            generator: raw.generator,
          }
        )
      }
      Err(_) => Err(ConfigError::Unknown)
    }
  }

  /// write the userdata into the file
  pub fn write(&self, path: &PathBuf) -> Result<(), ConfigError> {
    // create the raw data
    let raw = RawUser::from(self);
    // write the data
    raw.write_to_disk(&path.join(&self.username))
  }

  /// create new password
  pub fn new_password(&mut self) -> PasswordData {
    let data = PasswordData {
      login: None,
      password: None,
      url: None,
      description: None,
      uuid: Uuid::new_v4().to_string(),
      name: Some("Unnamed".to_string()),
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

  pub fn analyse_passwords(&self) -> AnalyseResult {
    // init vectors
    let mut reused = Vec::new();
    let mut very_strong = Vec::new();
    let mut strong = Vec::new();
    let mut medium = Vec::new();
    let mut weak = Vec::new();
    let mut very_weak = Vec::new();

    self.passwords.clone().into_iter().for_each(|ty| {
      if let PasswordType::Data(password) = ty.clone() {
        self.passwords.clone().into_iter().for_each(|cty| {
          if let PasswordType::Data(compare) = cty.clone() {
            if compare.password.as_ref().unwrap().eq(password.password.as_ref().unwrap()) && !compare.uuid.eq(&password.uuid) {
              reused.push(compare.uuid.clone());
              reused.push(password.uuid.clone());
            }
          }
        });

        // sort by strength
        match zxcvbn::zxcvbn(password.password.as_ref().unwrap().as_str(), &[]).unwrap().score() {
          0 => very_weak.push(password.uuid.clone()),
          1 => weak.push(password.uuid.clone()),
          2 => medium.push(password.uuid.clone()),
          3 => strong.push(password.uuid.clone()),
          4 => very_strong.push(password.uuid.clone()),
          // should never happen
          _ => {}
        }
      }
    });

    AnalyseResult {
      reused,
      very_strong,
      strong,
      medium,
      weak,
      very_weak,
    }
  }

  pub fn backup(&self) -> Option<Backup> {
    self.backup.clone()
  }

  pub fn passwords(&self) -> Vec<PasswordType> {
    self.passwords.clone()
  }
}

#[cfg(test)]
mod tests {
  use tempfile::{TempDir};
  use super::*;

  #[test]
  fn test_signup() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    User::new_from_signup(&dir.as_ref().to_path_buf(), data).unwrap();
    dir.close().unwrap();
  }

  #[test]
  fn test_login() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    let user = User::new_from_signup(&dir.as_ref().to_path_buf(), data.clone()).unwrap();

    let login = User::new_from_login(&dir.as_ref().to_path_buf(), data).unwrap();
    dir.close().unwrap();
    assert_eq!(user.username, login.username);
  }

  #[test]
  #[should_panic]
  fn test_auth() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    User::new_from_signup(&dir.as_ref().to_path_buf(), data.clone()).unwrap();

    User::new_from_login(&dir.as_ref().to_path_buf(), UserData {
      username: String::from("username"),
      password: String::from("te") }
    ).unwrap();
  }

  #[test]
  fn test_new_password() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    let mut user = User::new_from_signup(&dir.as_ref().to_path_buf(), data.clone()).unwrap();
    user.new_password();
    assert_eq!(1, user.passwords().len());
  }

  #[test]
  fn test_update_password() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    let mut user = User::new_from_signup(&dir.as_ref().to_path_buf(), data.clone()).unwrap();
    let mut password = user.new_password();
    password.password = Some(String::from("test"));
    user.update_password(password);

    assert_eq!(1, user.passwords().len());
    if let PasswordType::Data(pwd) = user.passwords().first().unwrap() {
      assert_eq!(&String::from("test"), pwd.password.as_ref().unwrap())
    }
    else { panic!("Wrong enum") }
  }

  fn test_delete_password() {
    let data = UserData {
      username: String::from("username"),
      password: String::from("password"),
    };
    let dir = TempDir::new().unwrap();
    let mut user = User::new_from_signup(&dir.as_ref().to_path_buf(), data.clone()).unwrap();
    let password = user.new_password();
    user.delete_password(password);
    assert_eq!(0, user.passwords().len());
  }
}
