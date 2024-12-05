use link::core::link::Guardable;
use link::guard::Auth;

#[tokio::test]
async fn test_auth_protection() {
    let key = b"test_key_12345";
    let auth = Auth::new(key);
    
    // Test protecting data
    let data = b"test message";
    let protected = auth.protect(data).await.unwrap();
    
    // Verify protected data format
    assert!(protected.len() > data.len());
    assert_eq!(protected.len(), data.len() + 32); // HMAC-SHA256 is 32 bytes
    
    // Test exposing protected data
    let exposed = auth.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
}

#[tokio::test]
async fn test_auth_invalid_data() {
    let key = b"test_key_12345";
    let auth = Auth::new(key);
    
    // Test data too short
    let invalid_data = vec![0u8; 16];
    let result = auth.expose(&invalid_data).await;
    assert!(result.is_err());
    
    // Test invalid signature
    let mut protected = auth.protect(b"test").await.unwrap();
    protected[0] ^= 1; // Flip one bit in signature
    let result = auth.expose(&protected).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auth_different_keys() {
    let auth1 = Auth::new(b"key1");
    let auth2 = Auth::new(b"key2");
    
    // Protect with first key
    let data = b"test message";
    let protected = auth1.protect(data).await.unwrap();
    
    // Try to expose with different key
    let result = auth2.expose(&protected).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auth_empty_data() {
    let auth = Auth::new(b"test_key");
    
    // Test protecting empty data
    let protected = auth.protect(&[]).await.unwrap();
    assert_eq!(protected.len(), 32); // Only signature
    
    // Test exposing empty data
    let exposed = auth.expose(&protected).await.unwrap();
    assert!(exposed.is_empty());
} 