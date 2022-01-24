#![cfg_attr(
all(not(debug_assertions), target_os = "linux"),
)]
#![feature(async_closure)]

mod model;
mod commands;

use std::fs;
use std::sync::{Arc, Mutex};
use tauri::api::path::{app_dir, BaseDirectory};
use tauri::{Config, generate_handler};
use crate::model::user::User;

pub struct UserState(Arc<Mutex<Option<User>>>);

fn main() {
  tauri::Builder::default()
    .setup(| app | {
      let path = app_dir(&*app.config()).unwrap();
      // create directory if does not exist
      fs::create_dir_all(path.as_path()).unwrap();

      Ok(())
    })
    // write empty user into the state
    .manage(UserState(Arc::new(Mutex::new(None))))
    .invoke_handler(generate_handler![
      commands::encryption::decrypt,
      commands::encryption::encrypt,
      commands::authentication::login,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
