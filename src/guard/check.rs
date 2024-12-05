use async_trait::async_trait;

use crate::core::error::{Error, Result};
use crate::core::link::Guardable;

pub struct Check {
    rules: Vec<Box<dyn Fn(&[u8]) -> bool + Send + Sync>>,
}

impl Check {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn add_rule<F>(&mut self, rule: F)
    where
        F: Fn(&[u8]) -> bool + Send + Sync + 'static,
    {
        self.rules.push(Box::new(rule));
    }
}

#[async_trait]
impl Guardable for Check {
    async fn protect(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Verify all rules pass
        for rule in &self.rules {
            if !rule(data) {
                return Err(Error::Guard("data validation failed".into()));
            }
        }
        Ok(data.to_vec())
    }

    async fn expose(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Re-verify rules on expose
        for rule in &self.rules {
            if !rule(data) {
                return Err(Error::Guard("data validation failed".into()));
            }
        }
        Ok(data.to_vec())
    }
} 