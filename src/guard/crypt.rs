use std::sync::Arc;
use async_trait::async_trait;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
use chacha20poly1305::aead::Aead;
use rand::{RngCore, thread_rng};

use crate::core::error::{Error, Result};
use crate::core::link::Guardable;

pub struct Crypt {
    key: Arc<[u8]>,
}

impl Crypt {
    pub fn new(key: &[u8]) -> Self {
        // Ensure key is exactly 32 bytes
        let mut fixed_key = [0u8; 32];
        let len = std::cmp::min(key.len(), 32);
        fixed_key[..len].copy_from_slice(&key[..len]);
        
        Self {
            key: Arc::from(&fixed_key[..]),
        }
    }

    fn create_cipher(&self) -> ChaCha20Poly1305 {
        let key = Key::from_slice(&self.key);
        ChaCha20Poly1305::new(key)
    }

    fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}

#[async_trait]
impl Guardable for Crypt {
    async fn protect(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.create_cipher();
        let nonce = Self::generate_nonce();
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| Error::Guard(e.to_string()))?;

        let mut protected = Vec::with_capacity(12 + ciphertext.len());
        protected.extend_from_slice(nonce.as_slice());
        protected.extend(ciphertext);
        Ok(protected)
    }

    async fn expose(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            return Err(Error::Guard("invalid data length".into()));
        }

        let (nonce, ciphertext) = data.split_at(12);
        let cipher = self.create_cipher();
        let nonce = Nonce::from_slice(nonce);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| Error::Guard(e.to_string()))
    }
} 