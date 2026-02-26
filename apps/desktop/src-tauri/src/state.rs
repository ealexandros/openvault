use openvault_sdk::Vault;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
}

pub type TauriState<'a> = State<'a, AppState>;
