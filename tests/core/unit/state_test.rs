use link::core::state::{State, Mode};

#[tokio::test]
async fn test_state_mode() {
    let state = State::new();
    
    // Test initial mode
    assert!(matches!(state.mode().await.unwrap(), Mode::Init));
    
    // Test mode changes
    state.set_mode(Mode::Ready).await.unwrap();
    assert!(matches!(state.mode().await.unwrap(), Mode::Ready));
    
    state.set_mode(Mode::Active).await.unwrap();
    assert!(matches!(state.mode().await.unwrap(), Mode::Active));
    
    state.set_mode(Mode::Pause).await.unwrap();
    assert!(matches!(state.mode().await.unwrap(), Mode::Pause));
    
    state.set_mode(Mode::Close).await.unwrap();
    assert!(matches!(state.mode().await.unwrap(), Mode::Close));
}

#[tokio::test]
async fn test_state_measure() {
    let state = State::new();
    
    // Test initial measure
    let measure = state.measure().await.unwrap();
    assert_eq!(measure.send, 0);
    assert_eq!(measure.receive, 0);
    assert_eq!(measure.error, 0);
    assert!(measure.start_timestamp.is_none());
    
    // Test recording send
    state.record_send(100).await.unwrap();
    let measure = state.measure().await.unwrap();
    assert_eq!(measure.send, 100);
    assert!(measure.start_timestamp.is_some());
    
    // Test recording receive
    state.record_receive(50).await.unwrap();
    let measure = state.measure().await.unwrap();
    assert_eq!(measure.receive, 50);
    
    // Test recording error
    state.record_error().await.unwrap();
    let measure = state.measure().await.unwrap();
    assert_eq!(measure.error, 1);
}

#[tokio::test]
async fn test_state_concurrent_access() {
    use tokio::task;
    
    let state = State::new();
    let state1 = state.clone();
    let state2 = state.clone();
    
    // Spawn two tasks that modify state concurrently
    let task1 = task::spawn(async move {
        for _ in 0..100 {
            state1.record_send(1).await.unwrap();
        }
    });
    
    let task2 = task::spawn(async move {
        for _ in 0..100 {
            state2.record_receive(1).await.unwrap();
        }
    });
    
    // Wait for both tasks to complete
    let _ = tokio::try_join!(task1, task2).unwrap();
    
    // Verify final state
    let measure = state.measure().await.unwrap();
    assert_eq!(measure.send, 100);
    assert_eq!(measure.receive, 100);
} 