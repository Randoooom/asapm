#![cfg_attr(
all(not(debug_assertions), target_os = "linux"),
windows_subsystem = "linux"
)]
#![feature(async_closure)]

use crate::model::router::Router;

mod model;

fn main() {
  tauri::Builder::default()
    .register_uri_scheme_protocol("api", move | app, request | {
      Router::new()
        .execute(app, request)
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
