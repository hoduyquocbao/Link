use link::core::link::{Settings, Linkable, Movable};
use link::core::state::Mode;
use link::net::{Socket, Group, Route};
use link::net::route::Entry;

#[tokio::test]
async fn test_net_components_integration() {
    use tokio::time::{sleep, timeout, Duration};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::Mutex;
    use std::sync::Arc;
    
    let addr = "127.0.0.1:8087";
    let settings = Settings::default();
    
    // Start a TCP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Create route
    let route = Route::new(settings.clone());
    route.add("server".into(), Entry {
        addr: addr.into(),
        weight: 1,
    }).await.unwrap();
    
    // Create and start sockets
    let mut sockets = vec![];
    let mut server_streams = vec![];
    
    for _ in 0..2 {
        let mut socket = route.connect("server").await.unwrap();
        socket.start().await.unwrap();
        sockets.push(socket);
        let (stream, _) = timeout(Duration::from_secs(5), listener.accept()).await.unwrap().unwrap();
        server_streams.push(stream);
    }
    
    // Give connections time to establish
    sleep(Duration::from_millis(100)).await;
    
    // Test concurrent data transfer
    let sockets = Arc::new(Mutex::new(sockets));
    let server_streams = Arc::new(Mutex::new(server_streams));
    let mut handles = vec![];
    
    for i in 0..2 {
        let sockets = sockets.clone();
        let server_streams = server_streams.clone();
        
        handles.push(tokio::spawn(async move {
            let data = vec![i as u8 + 1; 4];
            
            // Get socket
            let mut sockets = sockets.lock().await;
            let socket = &mut sockets[i];
            
            // Get corresponding server stream
            let mut streams = server_streams.lock().await;
            let stream = streams.get_mut(i).unwrap();
            
            // Send data from client to server
            socket.send(&data).await.unwrap();
            
            // Wait for data to be transmitted
            sleep(Duration::from_millis(50)).await;
            
            // Read data on server side
            let mut server_buf = vec![0; 4];
            stream.read_exact(&mut server_buf).await.unwrap();
            assert_eq!(server_buf, data, "Server received incorrect data");
            
            // Echo data back
            stream.write_all(&server_buf).await.unwrap();
            
            // Wait for data to be transmitted
            sleep(Duration::from_millis(50)).await;
            
            // Read echo on client side
            let mut client_buf = vec![0; 4];
            socket.receive(&mut client_buf).await.unwrap();
            assert_eq!(client_buf, data, "Client received incorrect data");
        }));
    }
    
    // Wait for all operations to complete with timeout
    for handle in handles {
        timeout(Duration::from_secs(10), handle).await.unwrap().unwrap();
    }
    
    // Stop all sockets
    let mut sockets = sockets.lock().await;
    for socket in sockets.iter_mut() {
        socket.stop().await.unwrap();
    }
}

#[tokio::test]
async fn test_net_error_handling() {
    let settings = Settings::default();
    let route = Route::new(settings.clone());
    
    // Test connecting to non-existent server
    route.add("invalid".into(), Entry {
        addr: "127.0.0.1:9999".into(),
        weight: 1,
    }).await.unwrap();
    
    let result = route.connect("invalid").await;
    assert!(result.is_err());
    
    // Test getting from empty group
    let mut group = Group::new(1, settings);
    let result = group.get().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_net_state_propagation() {
    let addr = "127.0.0.1:8088";
    let settings = Settings::default();
    
    // Start a TCP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Create and start group
    let mut group = Group::new(1, settings.clone());
    group.start().await.unwrap();
    assert!(matches!(group.state().await.unwrap(), Mode::Ready));
    
    // Add socket
    let mut socket = Socket::connect(addr, settings).await.unwrap();
    let (_stream, _) = listener.accept().await.unwrap();
    
    // Test state changes propagate
    socket.start().await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Ready));
    
    group.add(socket).await.unwrap();
    group.stop().await.unwrap();
    assert!(matches!(group.state().await.unwrap(), Mode::Close));
} 