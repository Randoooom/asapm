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

use crate::model::encryption::{Encryption, EncryptionError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Backup {
  // the uuid will be encrypted with the user password for better security on the local disk
  // will be available in plaintext here, not sure how secure that is
  uuid: String,
  enabled: bool,
  // the aes iv
  iv: String,
}

impl Backup {
  pub fn uuid(self) -> String {
    self.uuid.clone()
  }

  pub fn enabled(self) -> bool {
    self.enabled.clone()
  }

  /// update and decrypt the data with the encryption
  pub fn init_from_login(mut self, encryption: &Encryption) -> Result<Self, EncryptionError> {
    // decrypt uuid
    self.uuid = encryption.decrypt(self.uuid.clone(), self.iv.clone())?;
    // return the updated version
    Ok(self)
  }
}
