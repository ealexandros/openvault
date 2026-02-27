mod commands;
mod errors;
mod state;

use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            crate::commands::vault::create_vault,
            crate::commands::vault::open_vault,
            crate::commands::filesystem::browse_vault,
            crate::commands::filesystem::create_folder,
            crate::commands::filesystem::delete_item,
            crate::commands::filesystem::rename_item,
            crate::commands::filesystem::upload_file,
            crate::commands::filesystem::get_file_content,
            crate::commands::filesystem::path_is_file
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
