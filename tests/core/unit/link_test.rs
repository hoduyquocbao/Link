use link::core::link::{Link, Settings, Linkable, Movable};
use link::core::error::Error;
use link::core::state::Mode;

#[tokio::test]
async fn test_link_creation() {
    let settings = Settings::default();
    let link = Link::new(settings);
    assert!(matches!(link.state().await.unwrap(), Mode::Init));
}

#[tokio::test]
async fn test_link_lifecycle() {
    let mut link = Link::new(Settings::default());
    
    // Test initial state
    assert!(matches!(link.state().await.unwrap(), Mode::Init));
    
    // Test start
    link.start().await.unwrap();
    assert!(matches!(link.state().await.unwrap(), Mode::Ready));
    
    // Test stop
    link.stop().await.unwrap();
    assert!(matches!(link.state().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_link_data_transfer() {
    let mut link = Link::new(Settings::default());
    link.start().await.unwrap();
    
    // Test send
    let data = vec![1, 2, 3, 4];
    let sent = link.send(&data).await.unwrap();
    assert_eq!(sent, data.len());
    
    // Test receive
    let mut buf = vec![0; 4];
    let received = link.receive(&mut buf).await.unwrap();
    assert_eq!(received, buf.len());
}

#[tokio::test]
async fn test_link_size_limits() {
    let settings = Settings {
        size: 4,
        ..Settings::default()
    };
    let mut link = Link::new(settings);
    link.start().await.unwrap();
    
    // Test send with too large data
    let data = vec![0; 8];
    let result = link.send(&data).await;
    assert!(matches!(result, Err(Error::Net(_))));
    
    // Test receive with too large buffer
    let mut buf = vec![0; 8];
    let result = link.receive(&mut buf).await;
    assert!(matches!(result, Err(Error::Net(_))));
}

#[tokio::test]
async fn test_link_handler() {
    use link::core::link::Handler;
    
    struct TestHandler;
    
    #[async_trait::async_trait]
    impl Handler for TestHandler {
        async fn handle(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
            Ok(data.to_vec())
        }
    }
    
    let mut link = Link::new(Settings::default());
    link.add_handler(TestHandler);
    link.start().await.unwrap();
    
    let data = vec![1, 2, 3, 4];
    let sent = link.send(&data).await.unwrap();
    assert_eq!(sent, data.len());
} 