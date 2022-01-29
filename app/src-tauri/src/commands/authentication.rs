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

use tauri::{State, command, AppHandle, Wry};
use tauri::api::path::app_dir;
use crate::model::user::UserData;
use crate::{User, UserState};

#[command]
pub fn login(data: UserData, state: State<'_, UserState>, handle: AppHandle<Wry>) -> Result<(), ()> {
  // create the user from the data
  match User::new_from_login(&app_dir(&*handle.config()).unwrap(), data) {
    Ok(user) => {
      // update data in state
      *state.0.lock().unwrap() = Some(user);
      Ok(())
    }
    Err(_) => Err(())
  }
}

#[command]
pub fn signup(data: UserData, state: State<'_, UserState>, handle: AppHandle<Wry>) -> Result<(), ()> {
  // prevent already existing users creating new ones
  if let Some(_) = *state.0.lock().unwrap() {
    return Err(());
  }
  // try the signup
  match User::new_from_signup(&app_dir(&*handle.config()).unwrap(), data) {
    Ok(user) => {
      // update the user
      *state.0.lock().unwrap() = Some(user);
      Ok(())
    }
    Err(_) => Err(())
  }
}

#[command]
pub fn logout(state: State<'_, UserState>) {
  *state.0.lock().unwrap() = None;
}
