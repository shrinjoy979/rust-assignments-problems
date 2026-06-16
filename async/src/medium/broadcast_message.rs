/*
  Problem 89: Message Passing — Broadcast Channel

  Write an async function broadcast_demo() that creates a
  tokio::sync::broadcast channel. Spawn 3 tasks that each subscribe to the
  channel and receive a message. The main function should send a message
  and ensure all tasks receive it. Return the sum of received values.

  Run the tests for this problem with:
    cargo test --test broadcast_message_test
*/

use tokio::sync::broadcast;

pub async fn broadcast_demo() -> i32 {
    let (tx, _) = broadcast::channel(16);

    let mut handles = Vec::new();

    for _ in 0..3 {
        let mut rx = tx.subscribe();

        let handle = tokio::spawn(async move {
            rx.recv().await.unwrap()
        });

        handles.push(handle);
    }

    tx.send(10).unwrap();

    let mut sum = 0;

    for handle in handles {
        sum += handle.await.unwrap();
    }

    sum
}
