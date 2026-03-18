use openvault_sdk::Vault;
use std::{collections::HashMap, sync::Mutex};
use tauri::State;

#[derive(Default)]
pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub inner: Mutex<HashMap<String, Vec<u8>>>,
}

pub type TauriState<'a> = State<'a, AppState>;
