use tempfile::tempdir;
use link::store::File;

#[tokio::test]
async fn test_file_basic_operations() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    
    // Test write and read
    let path = "test.txt";
    let data = b"test content".to_vec();
    file_store.write(path, &data).await.unwrap();
    
    let result = file_store.read(path).await.unwrap();
    assert_eq!(result, data);
    
    // Test exists
    assert!(file_store.exists(path).await.unwrap());
    assert!(!file_store.exists("non_existent.txt").await.unwrap());
    
    // Test remove
    file_store.remove(path).await.unwrap();
    assert!(!file_store.exists(path).await.unwrap());
}

#[tokio::test]
async fn test_file_nested_directories() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    
    // Test nested path
    let path = "dir1/dir2/test.txt";
    let data = b"test content".to_vec();
    file_store.write(path, &data).await.unwrap();
    
    // Verify file exists and content is correct
    assert!(file_store.exists(path).await.unwrap());
    let result = file_store.read(path).await.unwrap();
    assert_eq!(result, data);
    
    // Verify directory structure
    let full_path = temp_dir.path().join("dir1").join("dir2").join("test.txt");
    assert!(full_path.exists());
}

#[tokio::test]
async fn test_file_overwrite() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    let path = "test.txt";
    
    // Write initial content
    file_store.write(path, b"initial content").await.unwrap();
    
    // Overwrite with new content
    let new_data = b"new content".to_vec();
    file_store.write(path, &new_data).await.unwrap();
    
    // Verify new content
    let result = file_store.read(path).await.unwrap();
    assert_eq!(result, new_data);
}

#[tokio::test]
async fn test_file_error_handling() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    
    // Test reading non-existent file
    let result = file_store.read("non_existent.txt").await;
    assert!(result.is_err());
    
    // Test removing non-existent file
    let result = file_store.remove("non_existent.txt").await;
    assert!(result.is_err());
    
    // Test writing to invalid path
    let result = file_store.write("", b"test").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_file_concurrent_access() {
    use tokio::task;
    
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    let file_store1 = file_store.clone();
    let file_store2 = file_store.clone();
    
    // Spawn two tasks that write files concurrently
    let task1 = task::spawn(async move {
        for i in 0..10 {
            let path = format!("file{}.txt", i);
            let data = format!("content{}", i).into_bytes();
            file_store1.write(&path, &data).await.unwrap();
        }
    });
    
    let task2 = task::spawn(async move {
        for i in 10..20 {
            let path = format!("file{}.txt", i);
            let data = format!("content{}", i).into_bytes();
            file_store2.write(&path, &data).await.unwrap();
        }
    });
    
    // Wait for both tasks to complete
    let _ = tokio::try_join!(task1, task2).unwrap();
    
    // Verify all files
    for i in 0..20 {
        let path = format!("file{}.txt", i);
        let expected = format!("content{}", i).into_bytes();
        let result = file_store.read(&path).await.unwrap();
        assert_eq!(result, expected);
    }
}

#[tokio::test]
async fn test_file_empty_data() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    let path = "empty.txt";
    
    // Write empty data
    file_store.write(path, b"").await.unwrap();
    
    // Verify file exists and is empty
    assert!(file_store.exists(path).await.unwrap());
    let result = file_store.read(path).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_file_large_data() {
    let temp_dir = tempdir().unwrap();
    let file_store = File::new(temp_dir.path());
    let path = "large.txt";
    
    // Create large data (1MB)
    let data = vec![0u8; 1024 * 1024];
    
    // Write and read large data
    file_store.write(path, &data).await.unwrap();
    let result = file_store.read(path).await.unwrap();
    
    assert_eq!(result.len(), data.len());
    assert_eq!(result, data);
} 