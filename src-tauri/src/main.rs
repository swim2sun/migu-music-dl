#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::Page;
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let context = tauri::generate_context!();
    // let menu = tauri::Menu::os_default(&context.package_info().name);
    let app_name = &context.package_info().name;
    let mut menu = Menu::new();
    menu = menu.add_submenu(Submenu::new(
        app_name,
        Menu::new()
            .add_native_item(MenuItem::About(
                app_name.to_string(),
                AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new("preferences", "Preferences...")
                    .accelerator("CmdOrCtrl+,")
                    .into(),
            )
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    ));
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "preferences" => {
                let window = event.window();
                window
                    .emit(
                        "show-preferences",
                        Payload {
                            message: "Hello".to_string(),
                        },
                    )
                    .unwrap();
            }
            _ => {}
        })
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

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
