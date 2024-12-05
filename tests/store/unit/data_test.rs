use link::store::Data;

#[tokio::test]
async fn test_data_basic_operations() {
    let data = Data::new();
    
    // Test set and get
    let key = "test_key";
    let value = b"test value".to_vec();
    data.set(key, value.clone()).await.unwrap();
    
    let result = data.get(key).await.unwrap();
    assert_eq!(result, Some(value));
    
    // Test non-existent key
    let result = data.get("non_existent").await.unwrap();
    assert_eq!(result, None);
    
    // Test remove
    data.remove(key).await.unwrap();
    let result = data.get(key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_data_clear() {
    let data = Data::new();
    
    // Add multiple entries
    data.set("key1", b"value1".to_vec()).await.unwrap();
    data.set("key2", b"value2".to_vec()).await.unwrap();
    data.set("key3", b"value3".to_vec()).await.unwrap();
    
    // Verify entries exist
    assert!(data.get("key1").await.unwrap().is_some());
    assert!(data.get("key2").await.unwrap().is_some());
    assert!(data.get("key3").await.unwrap().is_some());
    
    // Clear all entries
    data.clear().await.unwrap();
    
    // Verify all entries are removed
    assert!(data.get("key1").await.unwrap().is_none());
    assert!(data.get("key2").await.unwrap().is_none());
    assert!(data.get("key3").await.unwrap().is_none());
}

#[tokio::test]
async fn test_data_overwrite() {
    let data = Data::new();
    let key = "test_key";
    
    // Set initial value
    data.set(key, b"initial value".to_vec()).await.unwrap();
    
    // Overwrite with new value
    let new_value = b"new value".to_vec();
    data.set(key, new_value.clone()).await.unwrap();
    
    // Verify new value
    let result = data.get(key).await.unwrap();
    assert_eq!(result, Some(new_value));
}

#[tokio::test]
async fn test_data_concurrent_access() {
    use tokio::task;
    
    let data = Data::new();
    let data1 = data.clone();
    let data2 = data.clone();
    
    // Spawn two tasks that modify data concurrently
    let task1 = task::spawn(async move {
        for i in 0..100 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            data1.set(&key, value).await.unwrap();
        }
    });
    
    let task2 = task::spawn(async move {
        for i in 100..200 {
            let key = format!("key{}", i);
            let value = format!("value{}", i).into_bytes();
            data2.set(&key, value).await.unwrap();
        }
    });
    
    // Wait for both tasks to complete
    let _ = tokio::try_join!(task1, task2).unwrap();
    
    // Verify all entries
    for i in 0..200 {
        let key = format!("key{}", i);
        let expected = format!("value{}", i).into_bytes();
        let result = data.get(&key).await.unwrap();
        assert_eq!(result, Some(expected));
    }
}

#[tokio::test]
async fn test_data_empty_values() {
    let data = Data::new();
    
    // Test empty value
    data.set("empty", vec![]).await.unwrap();
    let result = data.get("empty").await.unwrap();
    assert_eq!(result, Some(vec![]));
    
    // Test empty key (though not recommended in practice)
    data.set("", b"value".to_vec()).await.unwrap();
    let result = data.get("").await.unwrap();
    assert_eq!(result, Some(b"value".to_vec()));
} 