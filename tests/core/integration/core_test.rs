use std::sync::Arc;
use tokio::sync::Mutex;
use link::core::link::{Link, Settings, Linkable, Movable, Handler};
use link::core::error::Error;
use link::core::state::Mode;

struct EchoHandler;

#[async_trait::async_trait]
impl Handler for EchoHandler {
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }
}

struct ReverseHandler;

#[async_trait::async_trait]
impl Handler for ReverseHandler {
    async fn handle(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut reversed = data.to_vec();
        reversed.reverse();
        Ok(reversed)
    }
}

#[tokio::test]
async fn test_link_with_handlers() {
    let mut link = Link::new(Settings::default());
    
    // Add multiple handlers
    link.add_handler(EchoHandler);
    link.add_handler(ReverseHandler);
    
    // Start link
    link.start().await.unwrap();
    assert!(matches!(link.state().await.unwrap(), Mode::Ready));
    
    // Test data transfer
    let data = vec![1, 2, 3, 4];
    let sent = link.send(&data).await.unwrap();
    assert_eq!(sent, data.len());
    
    let mut buf = vec![0; 4];
    let received = link.receive(&mut buf).await.unwrap();
    assert_eq!(received, buf.len());
    
    // Stop link
    link.stop().await.unwrap();
    assert!(matches!(link.state().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_link_error_handling() {
    let mut link = Link::new(Settings::default());
    
    // Test sending data when not ready
    let data = vec![1, 2, 3, 4];
    let result = link.send(&data).await;
    match result {
        Ok(_) => panic!("Expected error when sending data in init state"),
        Err(e) => println!("Got error: {:?}", e),
    }
    
    // Test receiving data when not ready
    let mut buf = vec![0; 4];
    let result = link.receive(&mut buf).await;
    match result {
        Ok(_) => panic!("Expected error when receiving data in init state"),
        Err(e) => println!("Got error: {:?}", e),
    }
    
    // Test invalid handler
    struct ErrorHandler;
    #[async_trait::async_trait]
    impl Handler for ErrorHandler {
        async fn handle(&self, _: &[u8]) -> Result<Vec<u8>, Error> {
            Err(Error::Guard("test error".into()))
        }
    }
    
    link.add_handler(ErrorHandler);
    
    // Start link with error handler
    link.start().await.unwrap();
    
    // Send data through error handler
    let result = link.send(&data).await;
    assert!(matches!(result, Err(Error::Guard(_))), "Expected Guard error from handler");
    
    // Stop link
    link.stop().await.unwrap();
    
    // Try to send data after stopping
    let result = link.send(&data).await;
    match result {
        Ok(_) => panic!("Expected error when sending data in closed state"),
        Err(e) => println!("Got error: {:?}", e),
    }
}

#[tokio::test]
async fn test_link_concurrent_access() {
    use tokio::task;
    
    let settings = Settings {
        size: 1024,
        ..Settings::default()
    };
    let link = Arc::new(Mutex::new(Link::new(settings)));
    
    // Start link
    link.lock().await.start().await.unwrap();
    
    // Spawn multiple tasks that use link concurrently
    let mut handles = vec![];
    for i in 0..10 {
        let link = link.clone();
        handles.push(task::spawn(async move {
            let mut link = link.lock().await;
            let data = vec![i as u8; 4];
            link.send(&data).await.unwrap();
            
            let mut buf = vec![0; 4];
            link.receive(&mut buf).await.unwrap();
        }));
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Verify final state
    let link = link.lock().await;
    let state = link.state().await.unwrap();
    assert!(matches!(state, Mode::Ready));
} 