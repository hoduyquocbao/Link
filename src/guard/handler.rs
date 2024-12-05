use async_trait::async_trait;
use crate::core::link::{Handler, Guardable};
use crate::core::error::Result;
use crate::guard::{Auth, Crypt, Check};

#[async_trait]
impl Handler for Auth {
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.expose(data).await
    }
}

#[async_trait]
impl Handler for Crypt {
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.protect(data).await
    }
}

#[async_trait]
impl Handler for Check {
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }
} 