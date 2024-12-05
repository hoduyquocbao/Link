use link::core::link::Guardable;
use link::guard::{Auth, Crypt, Check};

#[tokio::test]
async fn test_guard_chain() {
    // Create guards
    let auth = Auth::new(b"auth_key_12345");
    let crypt = Crypt::new(b"crypt_key_12345_crypt_key_12345_crypt");
    let mut check = Check::new();
    check.add_rule(|data| data.len() <= 1000);
    
    // Test data
    let data = b"test message";
    
    // Apply guards in sequence: check -> crypt -> auth
    let checked = check.protect(data).await.unwrap();
    let encrypted = crypt.protect(&checked).await.unwrap();
    let protected = auth.protect(&encrypted).await.unwrap();
    
    // Verify and decrypt in reverse sequence: auth -> crypt -> check
    let decrypted = auth.expose(&protected).await.unwrap();
    let exposed = crypt.expose(&decrypted).await.unwrap();
    let original = check.expose(&exposed).await.unwrap();
    
    assert_eq!(original, data);
}

#[tokio::test]
async fn test_guard_chain_invalid_data() {
    let auth = Auth::new(b"auth_key_12345");
    let crypt = Crypt::new(b"crypt_key_12345_crypt_key_12345_crypt");
    let mut check = Check::new();
    check.add_rule(|data| data.len() <= 10); // Max 10 bytes
    
    // Test data exceeding check rule
    let data = b"this message is too long for the check rule";
    
    // Protection should fail at check
    let result = check.protect(data).await;
    assert!(result.is_err());
    
    // Test corrupted protected data
    let valid_data = b"test";
    let checked = check.protect(valid_data).await.unwrap();
    let encrypted = crypt.protect(&checked).await.unwrap();
    let mut protected = auth.protect(&encrypted).await.unwrap();
    
    // Corrupt the auth signature
    protected[0] ^= 1;
    
    // Exposure should fail at auth
    let result = auth.expose(&protected).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_guard_chain_different_keys() {
    // Create two sets of guards with different keys
    let auth1 = Auth::new(b"auth_key_1");
    let crypt1 = Crypt::new(b"crypt_key_1_crypt_key_1_crypt_key_1_c");
    let mut check1 = Check::new();
    check1.add_rule(|data| !data.is_empty());
    
    let auth2 = Auth::new(b"auth_key_2");
    let crypt2 = Crypt::new(b"crypt_key_2_crypt_key_2_crypt_key_2_c");
    let mut check2 = Check::new();
    check2.add_rule(|data| !data.is_empty());
    
    let data = b"test message";
    
    // Protect with first set
    let checked = check1.protect(data).await.unwrap();
    let encrypted = crypt1.protect(&checked).await.unwrap();
    let protected = auth1.protect(&encrypted).await.unwrap();
    
    // Try to expose with second set
    let result = auth2.expose(&protected).await;
    assert!(result.is_err());
    
    // Even if auth was bypassed, crypt would fail
    let result = crypt2.expose(&encrypted).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_guard_chain_empty_data() {
    let auth = Auth::new(b"auth_key");
    let crypt = Crypt::new(b"crypt_key_12345_crypt_key_12345_crypt");
    let mut check = Check::new();
    check.add_rule(|data| !data.is_empty()); // Require non-empty data
    
    let data = b"";
    
    // Should fail at the check stage
    let result = check.protect(data).await;
    assert!(result.is_err());
    
    // But empty data should work without check
    let encrypted = crypt.protect(data).await.unwrap();
    let protected = auth.protect(&encrypted).await.unwrap();
    
    let decrypted = auth.expose(&protected).await.unwrap();
    let exposed = crypt.expose(&decrypted).await.unwrap();
    assert_eq!(exposed, data);
} 