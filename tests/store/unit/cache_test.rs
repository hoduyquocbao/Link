use std::time::Duration;
use tokio::time::sleep;
use link::store::Cache;

#[tokio::test]
async fn test_cache_basic_operations() {
    let cache = Cache::new(Duration::from_secs(1));
    
    // Test set and get
    let key = "test_key";
    let value = b"test value".to_vec();
    cache.set(key, value.clone()).await.unwrap();
    
    let result = cache.get(key).await.unwrap();
    assert_eq!(result, Some(value));
    
    // Test non-existent key
    let result = cache.get("non_existent").await.unwrap();
    assert_eq!(result, None);
    
    // Test remove
    cache.remove(key).await.unwrap();
    let result = cache.get(key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_cache_expiration() {
    let ttl = Duration::from_millis(100);
    let cache = Cache::new(ttl);
    
    // Set value
    let key = "test_key";
    let value = b"test value".to_vec();
    cache.set(key, value).await.unwrap();
    
    // Verify value exists
    let result = cache.get(key).await.unwrap();
    assert!(result.is_some());
    
    // Wait for expiration
    sleep(ttl + Duration::from_millis(50)).await;
    
    // Value should be expired
    let result = cache.get(key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_cache_cleanup() {
    let ttl = Duration::from_millis(100);
    let cache = Cache::new(ttl);
    
    // Add multiple entries
    cache.set("key1", b"value1".to_vec()).await.unwrap();
    cache.set("key2", b"value2".to_vec()).await.unwrap();
    
    // Wait for expiration
    sleep(ttl + Duration::from_millis(50)).await;
    
    // Add another entry
    cache.set("key3", b"value3".to_vec()).await.unwrap();
    
    // Run cleanup
    cache.cleanup().await.unwrap();
    
    // Verify expired entries are removed and new entry remains
    assert!(cache.get("key1").await.unwrap().is_none());
    assert!(cache.get("key2").await.unwrap().is_none());
    assert!(cache.get("key3").await.unwrap().is_some());
}

#[tokio::test]
async fn test_cache_clear() {
    let cache = Cache::new(Duration::from_secs(1));
    
    // Add multiple entries
    cache.set("key1", b"value1".to_vec()).await.unwrap();
    cache.set("key2", b"value2".to_vec()).await.unwrap();
    cache.set("key3", b"value3".to_vec()).await.unwrap();
    
    // Verify entries exist
    assert!(cache.get("key1").await.unwrap().is_some());
    assert!(cache.get("key2").await.unwrap().is_some());
    assert!(cache.get("key3").await.unwrap().is_some());
    
    // Clear all entries
    cache.clear().await.unwrap();
    
    // Verify all entries are removed
    assert!(cache.get("key1").await.unwrap().is_none());
    assert!(cache.get("key2").await.unwrap().is_none());
    assert!(cache.get("key3").await.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_overwrite() {
    let cache = Cache::new(Duration::from_secs(1));
    let key = "test_key";
    
    // Set initial value
    cache.set(key, b"initial value".to_vec()).await.unwrap();
    
    // Overwrite with new value
    let new_value = b"new value".to_vec();
    cache.set(key, new_value.clone()).await.unwrap();
    
    // Verify new value
    let result = cache.get(key).await.unwrap();
    assert_eq!(result, Some(new_value));
}

#[tokio::test]
async fn test_cache_concurrent_access() {
    use tokio::task;
    
    let cache = Cache::new(Duration::from_secs(1));
    let cache1 = cache.clone();
    let cache2 = cache.clone();
    
    // Spawn two tasks that modify cache concurrently
    let task1 = task::spawn(async move {
        for i in 0..100 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            cache1.set(&key, value).await.unwrap();
        }
    });
    
    let task2 = task::spawn(async move {
        for i in 100..200 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            cache2.set(&key, value).await.unwrap();
        }
    });
    
    // Wait for both tasks to complete
    let _ = tokio::try_join!(task1, task2).unwrap();
    
    // Verify all entries
    for i in 0..200 {
        let key = format!("key{}", i);
        let expected = format!("value{}", i).into_bytes();
        let result = cache.get(&key).await.unwrap();
        assert_eq!(result, Some(expected));
    }
} 