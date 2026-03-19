mod commands;
mod errors;
mod internal;
mod protocols;
mod state;
mod tasks;

use tauri::Manager;

use crate::protocols::{handle_secure_protocol, secure};
use crate::state::AppState;
use crate::tasks::spawn_cache_ttl_cleaner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            crate::commands::vault::create_vault,
            crate::commands::vault::open_vault,
            crate::commands::vault::compact_vault,
            crate::commands::vault::lock_vault,
            crate::commands::filesystem::path_is_file,
            crate::commands::filesystem::browse_fs,
            crate::commands::filesystem::create_folder,
            crate::commands::filesystem::upload_file,
            crate::commands::filesystem::upload_folder,
            crate::commands::filesystem::expose_file_url,
            crate::commands::filesystem::set_folder_icon,
            crate::commands::filesystem::set_favorite_item,
            crate::commands::filesystem::delete_item,
            crate::commands::filesystem::rename_item,
            crate::commands::filesystem::export_file,
            crate::commands::filesystem::export_folder,
            crate::commands::messages::get_message_credentials,
            crate::commands::messages::create_message_credentials,
            crate::commands::messages::renew_message_credentials,
            crate::commands::messages::reset_message_credentials,
            crate::commands::messages::list_contacts,
            crate::commands::messages::add_contact,
            crate::commands::messages::rename_contact,
            crate::commands::messages::remove_contact,
            crate::commands::messages::encrypt_message,
            crate::commands::messages::decrypt_message,
            crate::commands::messages::encrypt_file,
            crate::commands::messages::decrypt_file,
        ])
        .register_uri_scheme_protocol(secure::PROTOCOL_SCHEME, move |app, request| {
            handle_secure_protocol(app.app_handle(), request.uri())
        })
        .setup(|app| {
            let state = app.state::<AppState>();
            spawn_cache_ttl_cleaner(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
