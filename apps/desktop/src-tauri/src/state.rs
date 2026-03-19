use openvault_sdk::{SecretVec, Vault};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::State;

use crate::internal::ttl_cache::TtlCache;

pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub secure_proto: Arc<Mutex<TtlCache<String, SecretVec>>>,
}

impl Default for AppState {
    fn default() -> Self {
        let ttl_cache = TtlCache::new(Duration::from_secs(10));

        Self {
            vault: Mutex::new(None),
            secure_proto: Arc::new(Mutex::new(ttl_cache)),
        }
    }
}

pub type TauriState<'a> = State<'a, AppState>;
