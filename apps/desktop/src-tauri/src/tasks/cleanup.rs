use std::time::Duration;
use tauri::async_runtime;

use crate::state::TauriState;

pub fn spawn_cache_ttl_cleaner(store: TauriState) {
    let store = store.secure_proto.clone();

    async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            if let Ok(mut cache) = store.lock() {
                cache.purge_expired();
            }
        }
    });
}
