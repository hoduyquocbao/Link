use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::core::error::{Error, Result};

#[derive(Clone)]
pub struct File {
    root: PathBuf,
}

impl File {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub async fn write(&self, path: &str, data: &[u8]) -> Result<()> {
        let path = self.root.join(path);
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::Store(e.to_string()))?;
        }

        let mut file = fs::File::create(&path)
            .await
            .map_err(|e| Error::Store(e.to_string()))?;

        file.write_all(data)
            .await
            .map_err(|e| Error::Store(e.to_string()))?;

        Ok(())
    }

    pub async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let path = self.root.join(path);
        
        let mut file = fs::File::open(&path)
            .await
            .map_err(|e| Error::Store(e.to_string()))?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .await
            .map_err(|e| Error::Store(e.to_string()))?;

        Ok(data)
    }

    pub async fn remove(&self, path: &str) -> Result<()> {
        let path = self.root.join(path);
        
        fs::remove_file(&path)
            .await
            .map_err(|e| Error::Store(e.to_string()))?;

        Ok(())
    }

    pub async fn exists(&self, path: &str) -> Result<bool> {
        let path = self.root.join(path);
        Ok(path.exists())
    }
} 