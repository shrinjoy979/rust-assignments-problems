/*
  Problem 91: Async Shared State — Mutex Vec

  Rewrite Problem 74 / 79 using tokio::sync::Mutex to share a Vec<i32>.
  Spawn 5 tasks, each pushing 10 numbers into the vector.
  Return the length of the final vector.

  Run the tests for this problem with:
    cargo test --test async_shared_vec_test
*/

use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn async_shared_vec() -> usize {
    let vec = Arc::new(Mutex::new(Vec::<i32>::new()));
    let mut handles = Vec::new();

    for i in 0..5 {
        let vec = Arc::clone(&vec);

        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let mut v = vec.lock().await;
                v.push(i * 10 + j);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let v = vec.lock().await;
    v.len()
}
