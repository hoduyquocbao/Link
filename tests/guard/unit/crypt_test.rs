use link::core::link::Guardable;
use link::guard::Crypt;

#[tokio::test]
async fn test_crypt_protection() {
    let key = b"test_key_12345_test_key_12345_test_k";
    let crypt = Crypt::new(key);
    
    // Test protecting data
    let data = b"test message";
    let protected = crypt.protect(data).await.unwrap();
    
    // Verify protected data format
    assert!(protected.len() > data.len());
    assert_eq!(protected.len(), data.len() + 12 + 16); // 12 byte nonce + 16 byte tag
    
    // Test exposing protected data
    let exposed = crypt.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
}

#[tokio::test]
async fn test_crypt_invalid_data() {
    let key = b"test_key_12345_test_key_12345_test_k";
    let crypt = Crypt::new(key);
    
    // Test data too short
    let invalid_data = vec![0u8; 8];
    let result = crypt.expose(&invalid_data).await;
    assert!(result.is_err());
    
    // Test corrupted data
    let mut protected = crypt.protect(b"test").await.unwrap();
    protected[12] ^= 1; // Flip one bit in ciphertext
    let result = crypt.expose(&protected).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_crypt_different_keys() {
    let key1 = b"key1_key1_key1_key1_key1_key1_key1_k";
    let key2 = b"key2_key2_key2_key2_key2_key2_key2_k";
    let crypt1 = Crypt::new(key1);
    let crypt2 = Crypt::new(key2);
    
    // Protect with first key
    let data = b"test message";
    let protected = crypt1.protect(data).await.unwrap();
    
    // Try to expose with different key
    let result = crypt2.expose(&protected).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_crypt_empty_data() {
    let key = b"test_key_12345_test_key_12345_test_k";
    let crypt = Crypt::new(key);
    
    // Test protecting empty data
    let protected = crypt.protect(&[]).await.unwrap();
    assert_eq!(protected.len(), 28); // 12 byte nonce + 16 byte tag
    
    // Test exposing empty data
    let exposed = crypt.expose(&protected).await.unwrap();
    assert!(exposed.is_empty());
}

#[tokio::test]
async fn test_crypt_key_size() {
    // Test short key
    let short_key = b"short";
    let crypt = Crypt::new(short_key);
    
    // Should still work with padded key
    let data = b"test message";
    let protected = crypt.protect(data).await.unwrap();
    let exposed = crypt.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
    
    // Test long key
    let long_key = b"this_is_a_very_long_key_that_exceeds_32_bytes";
    let crypt = Crypt::new(long_key);
    
    // Should still work with truncated key
    let protected = crypt.protect(data).await.unwrap();
    let exposed = crypt.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
} 