#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use models::{Author, PagedResponseData};

const URL: &'static str = "http://localhost:8000/v1";

#[tauri::command]
async fn authors(start: u64, limit: u64) -> PagedResponseData<Author> {
  async {
    reqwest::get(format!("{}/authors?start={}&limit={}", URL, start, limit))
      .await?
      .json()
      .await
  }
  .await
  .unwrap()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![authors])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
