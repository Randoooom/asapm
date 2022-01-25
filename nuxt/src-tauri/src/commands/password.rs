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

use tauri::{AppHandle, command, State, Wry};
use tauri::api::path::app_dir;
use crate::model::user::{PasswordType, PasswordData};
use crate::{User, UserState};

#[command]
pub fn get_passwords(state: State<'_, UserState>) -> Vec<PasswordType> {
  // get the user
  match &*state.0.lock().unwrap() {
    Some(user) => user.passwords(),
    None => Vec::new()
  }
}

#[command]
pub fn new_password(state: State<'_, UserState>, handle: AppHandle<Wry>) -> Result<PasswordData, ()> {
  // get the user out of the stat
  match &mut *state.0.lock().unwrap() {
    Some(user) => {
      let data = user.new_password();
      // save data
      match User::write_file(&app_dir(&*handle.config()).unwrap(), user) {
        Ok(()) => Ok(data),
        Err(_) => Err(())
      }
    }
    None => Err(())
  }
}

#[command]
pub fn update_password(data: PasswordData, state: State<'_, UserState>, handle: AppHandle<Wry>) -> Result<(), ()> {
  // get the user
  match &mut *state.0.lock().unwrap() {
    Some(user) => {
      user.update_password(data);
      // save data
      match User::write_file(&app_dir(&*handle.config()).unwrap(),user) {
        Ok(()) => Ok(()),
        Err(_) => Err(())
      }
    }
    None => Err(())
  }
}
