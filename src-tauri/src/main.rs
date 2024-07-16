// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod xlsx;
use tauri::api::dialog::FileDialogBuilder;
use xlsx::xlsx::{get_sheets_names, read_excel_to_hash_vector, read_sheet_to_hash_vector};
use mouse_rs::Mouse;

#[tauri::command]
fn get_mouse_position(wx: i32, wy:i32) -> String {


    let mouse = Mouse::new();
    let position = mouse.get_position().unwrap();
    let x = position.x - wx;
    let y = position.y - wy;
    return format!("{{\"x\": {}, \"y\": {}}}", x, y);


}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_excel_to_hash_vector,
            read_sheet_to_hash_vector,
            get_sheets_names,
            get_mouse_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
