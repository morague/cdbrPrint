#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod printer;
use printer::{Job, Printer};
use std::fs;
use std::path::PathBuf;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![printer])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn printer(buf: String) {
  println!("in backend print fn!");
  let mut printer: Printer = Printer::new(String::from("QL-720NW"));
  printer.config_dimension(62u32, 29u32);
  let mut job: Job = serde_json::from_str::<Job>(&buf).unwrap();
  job._to_img();
  let state = printer.print(job);
}