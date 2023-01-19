
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod vk;
use vk::auth::handlers::vk_authorize;
fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![vk_authorize])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
