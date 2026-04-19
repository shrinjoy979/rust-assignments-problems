use r#async::medium::retry_logic::retry_operation;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_success_first_try() {
    let result: Result<i32, &str> = retry_operation(|| async { Ok(42) }, 3).await;
    assert_eq!(result, Ok(42));
}

#[tokio::test]
async fn test_retry_success() {
    let count = Arc::new(Mutex::new(0));
    let result: Result<i32, &str> = retry_operation(|| {
        let count = Arc::clone(&count);
        async move {
            let mut c = count.lock().unwrap();
            *c += 1;
            if *c < 3 { Err("fail") } else { Ok(100) }
        }
    }, 5).await;
    assert_eq!(result, Ok(100));
    assert_eq!(*count.lock().unwrap(), 3);
}

#[tokio::test]
async fn test_max_retries_exceeded() {
    let result: Result<i32, &str> = retry_operation(|| async { Err("always fail") }, 2).await;
    assert_eq!(result, Err("always fail"));
}
