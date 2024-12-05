use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::core::error::Result;

#[derive(Clone)]
pub struct Data {
    store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let mut store = self.store.write().await;
        store.insert(key.to_string(), value);
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let store = self.store.read().await;
        Ok(store.get(key).cloned())
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut store = self.store.write().await;
        store.remove(key);
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut store = self.store.write().await;
        store.clear();
        Ok(())
    }
} 