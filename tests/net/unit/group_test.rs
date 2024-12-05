use std::sync::Arc;

use tokio::sync::Mutex;
use link::core::link::{Linkable, Movable, Settings};
use link::core::state::Mode;
use link::net::{Socket, Group};
use tokio::time::{timeout, Duration};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::sync::CancellationToken;
use tokio::task::JoinHandle;


#[tokio::test]
async fn test_group_lifecycle() {
    let settings = Settings::default();
    let mut group = Group::new(10, settings);
    
    // Test initial state
    assert!(matches!(group.state().await.unwrap(), Mode::Init));
    
    // Test start
    group.start().await.unwrap();
    assert!(matches!(group.state().await.unwrap(), Mode::Ready));
    
    // Test stop
    group.stop().await.unwrap();
    assert!(matches!(group.state().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_group_connection_pool() {
    println!("Starting test_group_connection_pool");
    
    // Create timeout for entire test
    let test_timeout = timeout(Duration::from_secs(10), async {
        println!("Test execution started");
        
        // Create mock server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        println!("Server listening on {}", addr);
        
        // Create cancellation token for cleanup
        let cancel_token = CancellationToken::new();
        
        // Spawn server handler
        let server_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                let mut connection_handles = vec![];
                
                loop {
                    tokio::select! {
                        accept_result = listener.accept() => {
                            match accept_result {
                                Ok((mut socket, addr)) => {
                                    println!("Accepted connection from {}", addr);
                                    let cancel_token = cancel_token.clone();
                                    let handle = tokio::spawn(async move {
                                        let mut buf = [0u8; 1024];
                                        loop {
                                            tokio::select! {
                                                read_result = socket.read(&mut buf) => {
                                                    match read_result {
                                                        Ok(0) => break,
                                                        Ok(n) => {
                                                            if let Err(_) = socket.write_all(&buf[..n]).await {
                                                                break;
                                                            }
                                                        }
                                                        Err(_) => break,
                                                    }
                                                }
                                                _ = cancel_token.cancelled() => {
                                                    println!("Connection handler cancelled");
                                                    break;
                                                }
                                            }
                                        }
                                        println!("Connection handler stopped");
                                    });
                                    connection_handles.push(handle);
                                }
                                Err(_) => break,
                            }
                        }
                        _ = cancel_token.cancelled() => {
                            println!("Server cancelled, stopping all connections");
                            break;
                        }
                    }
                }
                
                // Wait for all connection handlers to complete
                println!("Waiting for {} connection handlers to complete", connection_handles.len());
                for handle in connection_handles {
                    match timeout(Duration::from_millis(100), handle).await {
                        Ok(_) => println!("Connection handler completed"),
                        Err(_) => println!("Connection handler timed out"),
                    }
                }
                println!("All connection handlers stopped");
            }
        });

        // Create and start group
        let settings = Settings::default();
        let mut group = Group::new(2, settings.clone());
        
        println!("Starting group");
        timeout(Duration::from_millis(500), group.start()).await??;
        println!("Group started");

        // Add first socket
        println!("Adding first socket");
        let mut socket1 = timeout(
            Duration::from_millis(500),
            Socket::connect(addr.to_string(), settings.clone())
        ).await??;
        timeout(Duration::from_millis(500), socket1.start()).await??;
        timeout(Duration::from_millis(500), group.add(socket1)).await??;
        println!("First socket added");

        // Add second socket
        println!("Adding second socket");
        let mut socket2 = timeout(
            Duration::from_millis(500),
            Socket::connect(addr.to_string(), settings.clone())
        ).await??;
        timeout(Duration::from_millis(500), socket2.start()).await??;
        timeout(Duration::from_millis(500), group.add(socket2)).await??;
        println!("Second socket added");

        // Get and verify first socket
        println!("Getting first socket");
        let mut holder1 = timeout(Duration::from_millis(500), group.get()).await??;
        assert!(matches!(
            timeout(Duration::from_millis(500), holder1.socket().state()).await??,
            Mode::Ready
        ));
        println!("First socket verified");

        // Get and verify second socket
        println!("Getting second socket");
        let mut holder2 = timeout(Duration::from_millis(500), group.get()).await??;
        assert!(matches!(
            timeout(Duration::from_millis(500), holder2.socket().state()).await??,
            Mode::Ready
        ));
        println!("Second socket verified");

        // Try to get third socket (should fail)
        println!("Trying to get third socket");
        assert!(group.get().await.is_err());
        println!("Third socket failed as expected");

        // Release sockets
        println!("Releasing sockets");
        drop(holder1);
        drop(holder2);
        println!("Sockets released");

        // Cleanup in correct order
        println!("Starting cleanup");
        
        // 1. Cancel all connection handlers
        println!("Cancelling all connections");
        cancel_token.cancel();
        
        // 2. Stop group and its sockets
        println!("Stopping group");
        match timeout(Duration::from_millis(500), group.stop()).await {
            Ok(Ok(_)) => println!("Group stopped successfully"),
            Ok(Err(e)) => println!("Group stop error: {}", e),
            Err(_) => println!("Group stop timed out"),
        }
        
        // 3. Wait for server to stop
        println!("Waiting for server to stop");
        match timeout(Duration::from_millis(500), server_handle).await {
            Ok(Ok(_)) => println!("Server stopped successfully"),
            Ok(Err(e)) => println!("Server stop error: {}", e),
            Err(_) => println!("Server stop timed out"),
        }
        
        println!("Cleanup completed");
        Ok::<_, Box<dyn std::error::Error>>(())
    });

    // Wait for test completion
    match test_timeout.await {
        Ok(Ok(_)) => println!("Test completed successfully"),
        Ok(Err(e)) => panic!("Test failed: {}", e),
        Err(_) => panic!("Test timed out after 10 seconds"),
    }
}

// Cleanup guard to ensure resources are freed even if test panics or times out
struct CleanupGuard {
    group: Option<Group>,
    server_handle: JoinHandle<()>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    cancel_token: CancellationToken,
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        println!("Cleanup guard triggered");
        
        // 1. Stop group first
        if let Some(mut group) = self.group.take() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let _ = rt.block_on(timeout(
                Duration::from_millis(500),
                group.stop()
            ));
            println!("Group stopped in cleanup");
        }
        
        // 2. Cancel all operations
        self.cancel_token.cancel();
        println!("Operations cancelled");
        
        // 3. Send shutdown signal
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
            println!("Shutdown signal sent");
        }
        
        // 4. Abort server handle
        self.server_handle.abort();
        println!("Server handle aborted");
    }
}

#[tokio::test]
async fn test_group_concurrent_access() {
    println!("Starting test_group_concurrent_access");
    
    // Create timeout for entire test
    let test_timeout = timeout(Duration::from_secs(10), async {
        println!("Test execution started");
        
        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        
        // Create mock server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        println!("Server listening on {}", addr);
        
        // Create cancellation token
        let cancel_token = CancellationToken::new();
        let cancel_token_clone = cancel_token.clone();
        
        // Spawn server handler
        let server_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                let mut connection_handles = vec![];
                let mut shutdown = false;
                
                while !shutdown {
                    tokio::select! {
                        accept_result = listener.accept() => {
                            match accept_result {
                                Ok((mut socket, addr)) => {
                                    println!("Accepted connection from {}", addr);
                                    let cancel_token = cancel_token.clone();
                                    let handle = tokio::spawn(async move {
                                        let mut buf = [0u8; 1024];
                                        loop {
                                            tokio::select! {
                                                read_result = socket.read(&mut buf) => {
                                                    match read_result {
                                                        Ok(0) => {
                                                            println!("Connection closed by peer {}", addr);
                                                            break;
                                                        }
                                                        Ok(n) => {
                                                            println!("Read {} bytes from {}", n, addr);
                                                            if let Err(e) = socket.write_all(&buf[..n]).await {
                                                                println!("Write error for {}: {}", addr, e);
                                                                break;
                                                            }
                                                        }
                                                        Err(e) => {
                                                            println!("Read error for {}: {}", addr, e);
                                                            break;
                                                        }
                                                    }
                                                }
                                                _ = cancel_token.cancelled() => {
                                                    println!("Connection handler cancelled for {}", addr);
                                                    break;
                                                }
                                            }
                                        }
                                    });
                                    connection_handles.push(handle);
                                }
                                Err(e) => {
                                    eprintln!("Accept error: {}", e);
                                    break;
                                }
                            }
                        }
                        Ok(_) = &mut shutdown_rx => {
                            println!("Server received shutdown signal");
                            shutdown = true;
                        }
                        _ = cancel_token.cancelled() => {
                            println!("Server cancelled");
                            shutdown = true;
                        }
                        else => break
                    }
                }
                
                println!("Server shutting down, cancelling {} connection handlers", connection_handles.len());
                cancel_token.cancel();
                
                // Wait for all connection handlers to complete with timeout
                for (i, handle) in connection_handles.into_iter().enumerate() {
                    match timeout(Duration::from_millis(100), handle).await {
                        Ok(_) => println!("Connection handler {} completed", i),
                        Err(_) => println!("Connection handler {} timed out", i),
                    }
                }
                println!("Server shutdown complete");
            }
        });

        println!("Server spawned, creating group");

        // Create cleanup guard that will be run when test ends or times out
        let _cleanup_guard = CleanupGuard {
            group: None,
            server_handle,
            shutdown_tx: Some(shutdown_tx),
            cancel_token: cancel_token_clone,
        };
        
        // Create and start group
        let settings = Settings::default();
        let group = Arc::new(Mutex::new(Group::new(5, settings.clone())));
        
        // Start group
        println!("Starting group");
        {
            let mut group = group.lock().await;
            timeout(Duration::from_millis(500), group.start())
                .await
                .expect("Group start timeout")
                .expect("Group start failed");
        }
        println!("Group started");
        
        // Add sockets to group
        println!("Adding sockets to group");
        {
            let group = group.lock().await;
            for i in 0..5 {
                let mut socket = timeout(
                    Duration::from_millis(500),
                    Socket::connect(addr.to_string(), settings.clone())
                ).await.expect("Connect timeout").expect("Connect failed");
                
                timeout(Duration::from_millis(500), socket.start())
                    .await
                    .expect("Socket start timeout")
                    .expect("Socket start failed");
                    
                timeout(Duration::from_millis(500), group.add(socket))
                    .await
                    .expect("Add socket timeout")
                    .expect("Add socket failed");
                println!("Added socket {}", i);
            }
        }
        println!("All sockets added");
        
        // Spawn multiple tasks that use connections concurrently
        println!("Spawning concurrent tasks");
        let mut handles = vec![];
        for i in 0..10 {
            let group = group.clone();
            handles.push(tokio::spawn(async move {
                let mut group = group.lock().await;
                match timeout(Duration::from_millis(500), group.get()).await {
                    Ok(Ok(mut holder)) => {
                        println!("Task {} got connection", i);
                        let socket = holder.socket();
                        let data = vec![1u8; 10];
                        if let Err(e) = socket.send(&data).await {
                            println!("Task {} send error: {}", i, e);
                        }
                        holder.release();
                        println!("Task {} released connection", i);
                    }
                    Ok(Err(e)) => println!("Task {} get error: {}", i, e),
                    Err(_) => println!("Task {} get timeout", i),
                }
            }));
        }
        
        // Wait for all tasks to complete
        println!("Waiting for tasks to complete");
        for (i, handle) in handles.into_iter().enumerate() {
            match timeout(Duration::from_secs(1), handle).await {
                Ok(Ok(_)) => println!("Task {} completed", i),
                Ok(Err(e)) => println!("Task {} error: {}", i, e),
                Err(_) => println!("Task {} timeout", i),
            }
        }
        println!("All tasks completed");
        
        // Stop group
        println!("Stopping group");
        {
            let mut group = group.lock().await;
            timeout(Duration::from_millis(500), group.stop())
                .await
                .expect("Stop timeout")
                .expect("Stop failed");
        }
        println!("Group stopped");
        
        println!("Test execution completed");
    });

    // Wait for test to complete or timeout
    match test_timeout.await {
        Ok(_) => println!("Test completed successfully within 10 seconds"),
        Err(_) => panic!("Test timed out after 10 seconds"),
    }
} 