/*
  Problem 99: Async TCP Echo Server (Simplified)

  Write an async function that starts a mock TCP echo server using
  tokio::net::TcpListener on a given port. It should accept one connection,
  read exactly 5 bytes, and write them back. Return the bytes read.

  Run the tests for this problem with:
    cargo test --test echo_server_test
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};

pub async fn run_echo_server(port: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    let (socket, _) = listener.accept().await?;
    let (mut reader, mut writer) = socket.into_split();

    let mut all_bytes = Vec::new();
    let mut tmp = [0u8; 4096];

    loop {
        // Use a short timeout: if no data arrives within 200ms after last chunk,
        // assume the client is done sending for now.
        let result = timeout(Duration::from_millis(200), reader.read(&mut tmp)).await;
        match result {
            Ok(Ok(0)) => break,          // EOF: client closed write half
            Ok(Ok(n)) => {
                writer.write_all(&tmp[..n]).await?;
                writer.flush().await?;
                all_bytes.extend_from_slice(&tmp[..n]);
            }
            Ok(Err(e)) => return Err(Box::new(e)),
            Err(_) => break,             // Timeout: no more data coming
        }
    }

    writer.shutdown().await.ok();
    Ok(all_bytes)
}