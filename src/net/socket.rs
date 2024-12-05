use std::sync::Arc;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use async_trait::async_trait;

use crate::core::error::{Error, Result};
use crate::core::link::{Linkable, Movable, Settings, Handler};
use crate::core::state::Mode;

/// Network socket implementation
pub struct Socket {
    stream: TcpStream,
    _settings: Arc<Settings>,
    handlers: Vec<Box<dyn Handler>>,
}

impl Socket {
    pub async fn connect<A: ToSocketAddrs>(addr: A, settings: Settings) -> Result<Self> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| Error::Net(e.to_string()))?;

        Ok(Self {
            stream,
            _settings: Arc::new(settings),
            handlers: Vec::new(),
        })
    }

    pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }

    async fn process_outgoing(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut processed = data.to_vec();
        for handler in &self.handlers {
            processed = handler.handle(&processed).await?;
        }
        Ok(processed)
    }

    async fn process_incoming(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut processed = data.to_vec();
        for handler in self.handlers.iter().rev() {
            processed = handler.handle(&processed).await?;
        }
        Ok(processed)
    }
}

#[async_trait]
impl Linkable for Socket {
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    async fn state(&self) -> Result<Mode> {
        Ok(Mode::Ready)
    }
}

#[async_trait]
impl Movable for Socket {
    async fn send(&mut self, data: &[u8]) -> Result<usize> {
        // Process data through handlers
        let processed = self.process_outgoing(data).await?;
        
        // Send data length first (4 bytes)
        let len = processed.len() as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;
        
        // Send processed data
        self.stream.write_all(&processed).await?;
        self.stream.flush().await?;
        
        Ok(data.len())
    }

    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Read length prefix (4 bytes)
        let mut len_bytes = [0u8; 4];
        self.stream.read_exact(&mut len_bytes).await?;
        let len = u32::from_be_bytes(len_bytes) as usize;
        
        // Read exact amount of data
        let mut data = vec![0u8; len];
        self.stream.read_exact(&mut data).await?;
        
        // Process received data through handlers in reverse
        let processed = self.process_incoming(&data).await?;
        
        // Copy processed data to output buffer
        let copy_len = processed.len().min(buf.len());
        buf[..copy_len].copy_from_slice(&processed[..copy_len]);
        
        Ok(copy_len)
    }
} 