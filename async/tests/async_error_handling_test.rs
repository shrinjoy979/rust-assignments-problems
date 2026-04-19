use r#async::medium::async_error_handling::{fetch_and_process, fetch_data};

#[tokio::test]
async fn test_ok() {
    assert_eq!(fetch_and_process(10).await, Ok(7)); // "Data_10"
}

#[tokio::test]
async fn test_err() {
    assert_eq!(fetch_and_process(0).await, Err("Invalid ID".to_string()));
}

#[tokio::test]
async fn test_data_ok() {
    assert_eq!(fetch_data(1).await, Ok("Data_1".to_string()));
}
