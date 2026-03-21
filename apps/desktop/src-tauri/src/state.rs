use openvault_sdk::Vault;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::State;

use crate::internal::ttl_cache::TtlCache;
use crate::protocols::SecurePayload;

pub struct AppState {
    pub vault: Mutex<Option<Vault>>,
    pub secure_payloads: Arc<Mutex<TtlCache<String, SecurePayload>>>,
}

impl Default for AppState {
    fn default() -> Self {
        let ttl_cache = TtlCache::new(Duration::from_secs(10));

        Self {
            vault: Mutex::new(None),
            secure_payloads: Arc::new(Mutex::new(ttl_cache)),
        }
    }
}

pub type TauriState<'a> = State<'a, AppState>;
