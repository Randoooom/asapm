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
use crate::{UserState};
use crate::model::generator::PasswordGenerator;

#[command]
pub fn update_generator(state: State<'_, UserState>, generator: PasswordGenerator, handle: AppHandle<Wry>) -> Result<(), ()> {
  // get the user
  match &mut *state.0.lock().unwrap() {
    Some(user) => {
      user.update_generator(generator);
      // save
      match user.write(&app_dir(&*handle.config()).unwrap()) {
        Ok(()) => Ok(()),
        Err(_) => Err(())
      }
    }
    None => Err(())
  }
}

#[command]
pub fn get_generator(state: State<'_, UserState>) -> Result<PasswordGenerator, ()> {
  // get the user
  match &mut *state.0.lock().unwrap() {
    Some(user) => Ok(user.generator()),
    None => Err(())
  }
}

#[command]
pub fn generate_password(state: State<'_, UserState>, generator: Option<PasswordGenerator>) -> Result<String, ()> {
  // get the user
  match &mut *state.0.lock().unwrap() {
    Some(user) => Ok(user.generator().generate(generator)),
    None => Err(())
  }
}
