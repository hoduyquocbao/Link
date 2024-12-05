use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::core::error::Result;

#[derive(Clone)]
pub struct Cache {
    store: Arc<RwLock<HashMap<String, (Vec<u8>, Instant)>>>,
    ttl: Duration,
}

impl Cache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let mut store = self.store.write().await;
        store.insert(key.to_string(), (value, Instant::now()));
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut store = self.store.write().await;
        
        if let Some((value, time)) = store.get(key) {
            if time.elapsed() > self.ttl {
                store.remove(key);
                Ok(None)
            } else {
                Ok(Some(value.clone()))
            }
        } else {
            Ok(None)
        }
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

    pub async fn cleanup(&self) -> Result<()> {
        let mut store = self.store.write().await;
        store.retain(|_, (_, time)| time.elapsed() <= self.ttl);
        Ok(())
    }
} 