use std::sync::Arc;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::core::error::{Error, Result};
use crate::core::link::Guardable;

type HmacSha256 = Hmac<Sha256>;

const SIGNATURE_LENGTH: usize = 32;

pub struct Auth {
    key: Arc<[u8]>,
}

impl Auth {
    pub fn new(key: &[u8]) -> Self {
        Self {
            key: Arc::from(key),
        }
    }

    fn compute_hmac(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    fn verify_hmac(&self, data: &[u8], signature: &[u8]) -> bool {
        let computed = self.compute_hmac(data);
        if computed.len() != signature.len() {
            return false;
        }
        // Use constant-time comparison
        let mut result = 0u8;
        for (a, b) in computed.iter().zip(signature.iter()) {
            result |= a ^ b;
        }
        result == 0
    }
}

#[async_trait]
impl Guardable for Auth {
    async fn protect(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Compute HMAC signature
        let signature = self.compute_hmac(data);
        
        // Create protected data with signature
        let mut protected = Vec::with_capacity(SIGNATURE_LENGTH + data.len());
        protected.extend_from_slice(&signature);
        protected.extend_from_slice(data);
        
        Ok(protected)
    }

    async fn expose(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Verify data length
        if data.len() < SIGNATURE_LENGTH {
            return Err(Error::Guard("data too short for signature".into()));
        }
        
        // Split signature and content
        let (signature, content) = data.split_at(SIGNATURE_LENGTH);
        
        // Verify signature
        if !self.verify_hmac(content, signature) {
            return Err(Error::Guard("invalid signature".into()));
        }
        
        Ok(content.to_vec())
    }
} 