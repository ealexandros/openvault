use openvault_sdk::{SecretVec, Vault};
use std::{collections::HashMap, sync::Mutex};
use tauri::State;

// @todo-now add a cron job that sweeps the secure_proto

#[derive(Default)]
pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub secure_proto: Mutex<HashMap<String, SecretVec>>,
}

pub type TauriState<'a> = State<'a, AppState>;
