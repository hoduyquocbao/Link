use std::time::Duration;
use tempfile::tempdir;
use link::store::{Data, Cache, File};

#[tokio::test]
async fn test_store_chain() {
    let data = Data::new();
    let cache = Cache::new(Duration::from_secs(1));
    let temp_dir = tempdir().unwrap();
    let file = File::new(temp_dir.path());
    
    // Test data: store in memory, cache, and persist to file
    let key = "test_key";
    let value = b"test value".to_vec();
    
    // Store in memory
    data.set(key, value.clone()).await.unwrap();
    
    // Cache the value
    cache.set(key, value.clone()).await.unwrap();
    
    // Persist to file
    file.write(key, &value).await.unwrap();
    
    // Verify data is consistent across stores
    let data_result = data.get(key).await.unwrap();
    let cache_result = cache.get(key).await.unwrap();
    let file_result = file.read(key).await.unwrap();
    
    assert_eq!(data_result, Some(value.clone()));
    assert_eq!(cache_result, Some(value.clone()));
    assert_eq!(file_result, value);
}

#[tokio::test]
async fn test_store_cache_fallback() {
    let data = Data::new();
    let cache = Cache::new(Duration::from_millis(100));
    let temp_dir = tempdir().unwrap();
    let file = File::new(temp_dir.path());
    
    let key = "test_key";
    let value = b"test value".to_vec();
    
    // Store in all layers
    data.set(key, value.clone()).await.unwrap();
    cache.set(key, value.clone()).await.unwrap();
    file.write(key, &value).await.unwrap();
    
    // Wait for cache to expire
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Cache should be empty, fallback to data store
    assert!(cache.get(key).await.unwrap().is_none());
    assert_eq!(data.get(key).await.unwrap(), Some(value.clone()));
    
    // Clear data store, fallback to file
    data.clear().await.unwrap();
    assert!(data.get(key).await.unwrap().is_none());
    assert_eq!(file.read(key).await.unwrap(), value);
}

#[tokio::test]
async fn test_store_concurrent_operations() {
    use tokio::task;
    
    let data = Data::new();
    let cache = Cache::new(Duration::from_secs(1));
    let temp_dir = tempdir().unwrap();
    let file = File::new(temp_dir.path());
    
    // Clone stores for concurrent tasks
    let data1 = data.clone();
    let cache1 = cache.clone();
    let file1 = file.clone();
    
    let data2 = data.clone();
    let cache2 = cache.clone();
    let file2 = file.clone();
    
    // Spawn tasks that operate on different stores concurrently
    let task1 = task::spawn(async move {
        for i in 0..10 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            
            data1.set(&key, value.clone()).await.unwrap();
            cache1.set(&key, value.clone()).await.unwrap();
            file1.write(&key, &value).await.unwrap();
        }
    });
    
    let task2 = task::spawn(async move {
        for i in 10..20 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            
            data2.set(&key, value.clone()).await.unwrap();
            cache2.set(&key, value.clone()).await.unwrap();
            file2.write(&key, &value).await.unwrap();
        }
    });
    
    // Wait for tasks to complete
    let _ = tokio::try_join!(task1, task2).unwrap();
    
    // Verify data is consistent across all stores
    for i in 0..20 {
        let key = format!("key{}", i);
        let expected = format!("value{}", i).into_bytes();
        
        assert_eq!(data.get(&key).await.unwrap(), Some(expected.clone()));
        assert_eq!(cache.get(&key).await.unwrap(), Some(expected.clone()));
        assert_eq!(file.read(&key).await.unwrap(), expected);
    }
}

#[tokio::test]
async fn test_store_error_propagation() {
    let data = Data::new();
    let cache = Cache::new(Duration::from_secs(1));
    let temp_dir = tempdir().unwrap();
    let file = File::new(temp_dir.path());
    
    // Test non-existent key
    let key = "non_existent";
    assert!(data.get(key).await.unwrap().is_none());
    assert!(cache.get(key).await.unwrap().is_none());
    assert!(file.read(key).await.is_err());
    
    // Test invalid file operations
    assert!(file.write("", b"test").await.is_err());
    assert!(file.remove("non_existent").await.is_err());
    
    // Test cache expiration doesn't affect other stores
    let key = "test_key";
    let value = b"test value".to_vec();
    
    data.set(key, value.clone()).await.unwrap();
    cache.set(key, value.clone()).await.unwrap();
    file.write(key, &value).await.unwrap();
    
    tokio::time::sleep(Duration::from_millis(1100)).await;
    
    assert!(cache.get(key).await.unwrap().is_none());
    assert_eq!(data.get(key).await.unwrap(), Some(value.clone()));
    assert_eq!(file.read(key).await.unwrap(), value);
} 