#![feature(async_closure)]

extern crate core;

mod model;
mod commands;

use std::fs;
use std::sync::{Arc, Mutex};
use tauri::api::path::{app_dir};
use tauri::{generate_handler};
use crate::model::user::User;

pub struct UserState(Arc<Mutex<Option<User>>>);

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let path = app_dir(&*app.config()).unwrap();
      // create directory if does not exist
      fs::create_dir_all(path.as_path()).unwrap();

      Ok(())
    })
    // write empty user into the state
    .manage(UserState(Arc::new(Mutex::new(None))))
    .invoke_handler(generate_handler![
      commands::authentication::login,
      commands::authentication::signup,
      commands::authentication::logout,
      commands::password::new_password,
      commands::password::get_passwords,
      commands::password::update_password,
      commands::password::delete_password,
      commands::password::password_strength,
      commands::password::analyse,
      commands::generator::update_generator,
      commands::generator::get_generator,
      commands::generator::generate_password,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
