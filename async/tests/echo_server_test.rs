use r#async::hard::echo_server::run_echo_server;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_echo() {
    let port = 8080;
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        run_echo_server(port).await.unwrap()
    });

    // Wait a bit for server to start
    sleep(Duration::from_millis(50)).await;

    // Client connects
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"hello").await.unwrap();

    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"hello");

    let server_result = server_handle.await.unwrap();
    assert_eq!(server_result, b"hello".to_vec());
}
