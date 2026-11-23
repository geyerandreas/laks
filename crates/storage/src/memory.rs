use crate::error::Result;

use bytes::Bytes;
use futures::StreamExt;
use object_store::{ObjectStore, ObjectStoreExt, PutMode, memory::InMemory, path::Path};

#[derive(Debug, Clone)]
pub struct MemoryStorage(InMemory);

impl MemoryStorage {
    pub fn new() -> Self {
        Self(InMemory::new())
    }

    pub async fn put(&self, key: &str, data: impl Into<Bytes>) -> Result<()> {
        self.0
            .put_opts(&key.into(), data.into().into(), PutMode::Create.into())
            .await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Bytes> {
        Ok(self.0.get(&key.into()).await?.bytes().await?)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        Ok(self.0.delete(&key.into()).await?)
    }

    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>> {
        let prefix_path: Option<Path> = prefix.map(Path::from);
        let mut stream = self.0.list(prefix_path.as_ref());
        let mut out = Vec::new();

        while let Some(meta) = stream.next().await {
            let meta = meta?;
            out.push(meta.location.to_string());
        }

        Ok(out)
    }

    pub async fn exist(&self, key: &str) -> Result<bool> {
        self.0
            .head(&key.into())
            .await
            .map(|_| true)
            .or_else(|e| match e {
                object_store::Error::NotFound { .. } => Ok(false),
                _ => Err(e.into()),
            })
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn put_then_get_roundtrips() {
        let storage = MemoryStorage::new();
        storage.put("key1", b"Hello".to_vec()).await.unwrap();

        let data = storage.get("key1").await.unwrap();
        assert_eq!(data, Bytes::from_static(b"Hello"));
    }

    #[tokio::test]
    async fn missing_file_raises_error() {
        let storage = MemoryStorage::new();
        let error = storage.get("missingKey").await;
        assert!(error.is_err());
    }

    #[tokio::test]
    async fn put_fails_if_key_already_exists() {
        let storage = MemoryStorage::new();
        storage.put("key1", b"first".to_vec()).await.unwrap();

        let result = storage.put("key1", b"second".to_vec()).await;
        assert!(result.is_err());
    }
}
