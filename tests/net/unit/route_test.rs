use link::core::link::{Settings, Linkable};
use link::core::state::Mode;
use link::net::Route;
use link::net::route::Entry;

#[tokio::test]
async fn test_route_lifecycle() {
    let settings = Settings::default();
    let mut route = Route::new(settings);
    
    // Test initial state
    assert!(matches!(route.state().await.unwrap(), Mode::Init));
    
    // Test start
    route.start().await.unwrap();
    assert!(matches!(route.state().await.unwrap(), Mode::Ready));
    
    // Test stop
    route.stop().await.unwrap();
    assert!(matches!(route.state().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_route_table() {
    let settings = Settings::default();
    let route = Route::new(settings);
    
    // Add routes
    route.add("server1".into(), Entry {
        addr: "127.0.0.1:8084".into(),
        weight: 1,
    }).await.unwrap();
    
    route.add("server2".into(), Entry {
        addr: "127.0.0.1:8085".into(),
        weight: 2,
    }).await.unwrap();
    
    // Test get route
    let entry = route.get("server1").await.unwrap();
    assert_eq!(entry.addr, "127.0.0.1:8084");
    assert_eq!(entry.weight, 1);
    
    // Test list routes
    let routes = route.list().await.unwrap();
    assert_eq!(routes.len(), 2);
    
    // Test remove route
    route.remove("server1").await.unwrap();
    let result = route.get("server1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_route_connection() {
    let addr = "127.0.0.1:8086";
    let settings = Settings::default();
    let route = Route::new(settings);
    
    // Start a TCP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Add route
    route.add("test".into(), Entry {
        addr: addr.into(),
        weight: 1,
    }).await.unwrap();
    
    // Test connect
    let mut socket = route.connect("test").await.unwrap();
    let (_stream, _) = listener.accept().await.unwrap();
    
    // Test socket works
    socket.start().await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Ready));
    
    socket.stop().await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Close));
} 