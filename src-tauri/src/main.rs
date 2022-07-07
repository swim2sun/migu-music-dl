#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::Page;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![search, download])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn search(key_word: &str, page_number: u8, page_size: u8, quality: &str) -> Result<Page, String> {
    app::search(key_word, page_number, page_size, quality)
}

#[tauri::command]
async fn download(name: &str, url: &str, path: &str) -> Result<(), String> {
    app::download(name, url, path)
}