use link::core::link::{Settings, Linkable, Movable};
use link::core::state::Mode;
use link::net::Socket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_socket_connection() {
    let addr = "127.0.0.1:8080";
    let settings = Settings::default();
    
    // Start a TCP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Connect socket
    let mut socket = Socket::connect(addr, settings).await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Init));
    
    // Accept connection
    let (_stream, _) = listener.accept().await.unwrap();
    
    // Test lifecycle
    socket.start().await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Ready));
    
    socket.stop().await.unwrap();
    assert!(matches!(socket.state().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_socket_data_transfer() {
    let addr = "127.0.0.1:8081";
    let settings = Settings::default();
    
    // Start a TCP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Connect socket
    let mut socket = Socket::connect(addr, settings).await.unwrap();
    socket.start().await.unwrap();
    
    // Accept connection
    let (mut stream, _) = listener.accept().await.unwrap();
    
    // Test send
    let data = vec![1, 2, 3, 4];
    let sent = socket.send(&data).await.unwrap();
    assert_eq!(sent, data.len());
    
    // Use tokio's async read
    let mut buf = vec![0; 4];
    let received = stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(received, data.len());
    assert_eq!(buf, data);
    
    // Test receive
    stream.write_all(&data).await.unwrap();
    let mut buf = vec![0; 4];
    let received = socket.receive(&mut buf).await.unwrap();
    assert_eq!(received, data.len());
    assert_eq!(buf, data);
} 