use link::core::link::Guardable;
use link::guard::Check;

#[tokio::test]
async fn test_check_no_rules() {
    let check = Check::new();
    
    // Test with no rules
    let data = b"test message";
    let protected = check.protect(data).await.unwrap();
    assert_eq!(protected, data);
    
    let exposed = check.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
}

#[tokio::test]
async fn test_check_single_rule() {
    let mut check = Check::new();
    
    // Add rule: data must not be empty
    check.add_rule(|data| !data.is_empty());
    
    // Test valid data
    let data = b"test message";
    let protected = check.protect(data).await.unwrap();
    assert_eq!(protected, data);
    
    let exposed = check.expose(&protected).await.unwrap();
    assert_eq!(exposed, data);
    
    // Test invalid data
    let empty = b"";
    let result = check.protect(empty).await;
    assert!(result.is_err());
    
    let result = check.expose(empty).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_check_multiple_rules() {
    let mut check = Check::new();
    
    // Add multiple rules
    check.add_rule(|data| data.len() >= 4); // Min length
    check.add_rule(|data| data.len() <= 100); // Max length
    check.add_rule(|data| data.iter().all(|&b| b.is_ascii())); // ASCII only
    
    // Test valid data
    let data = b"test message";
    let protected = check.protect(data).await.unwrap();
    assert_eq!(protected, data);
    
    // Test data too short
    let short = b"abc";
    let result = check.protect(short).await;
    assert!(result.is_err());
    
    // Test data too long
    let long = vec![b'a'; 101];
    let result = check.protect(&long).await;
    assert!(result.is_err());
    
    // Test non-ASCII data
    let non_ascii = &[0xFF, 0xFF, 0xFF, 0xFF];
    let result = check.protect(non_ascii).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_check_complex_rule() {
    let mut check = Check::new();
    
    // Add complex rule: data must be valid UTF-8 and contain "test"
    check.add_rule(|data| {
        if let Ok(s) = std::str::from_utf8(data) {
            s.contains("test")
        } else {
            false
        }
    });
    
    // Test valid data
    let data = b"this is a test message";
    let protected = check.protect(data).await.unwrap();
    assert_eq!(protected, data);
    
    // Test invalid UTF-8
    let invalid_utf8 = &[0xFF, 0xFF, 0xFF, 0xFF];
    let result = check.protect(invalid_utf8).await;
    assert!(result.is_err());
    
    // Test valid UTF-8 but no "test"
    let no_test = b"this is a message";
    let result = check.protect(no_test).await;
    assert!(result.is_err());
} 