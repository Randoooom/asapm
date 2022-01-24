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

use tauri::{State, command};
use crate::{User, UserState};
use crate::model::encryption::{CipherText, Encryption, EncryptionError};

#[command]
pub fn decrypt(data: String, state: State<'_, UserState>) -> Result<String, ()> {
  // get the user
  match *&state.0.lock().unwrap().as_ref() {
    Some(user) => {
      // decrypt the data
      match user.encryption.decrypt(data.as_str()) {
        Ok(decrypted) => Ok(decrypted),
        Err(_) => Err(())
      }
    }
    // throw err on logged out
    None => Err(())
  }
}

#[command]
pub fn encrypt(data: String, state: State<'_, UserState>) -> Result<CipherText, ()> {
  // get the user
  match *&state.0.lock().unwrap().as_ref() {
    Some(user) => {
      // encrypt the data
      match user.encryption.encrypt(data.as_str()) {
        Ok(encrypted) => Ok(encrypted),
        Err(_) => Err(())
      }
    },
    None => Err(())
  }
}
