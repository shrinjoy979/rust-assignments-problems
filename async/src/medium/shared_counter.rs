/*
  Problem 74: Shared State — Mutex Counter

  Write a function that spawns 10 threads. Each thread should increment a shared
  counter 100 times. Use Arc<Mutex<i32>> to share the counter safely.
  Wait for all threads to finish and return the final counter value.

  Run the tests for this problem with:
    cargo test --test shared_counter_test
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn multithreaded_counter() -> i32 {
  // Create the shared counter wrapped in Arc<Mutex<i32>>
    let counter = Arc::new(Mutex::new(0));
    
    // Hold handles to all spawned threads so we can wait for them
    let mut handles = vec![];
    
    // Spawn 10 threads
    for _ in 0..10 {
        // Clone the Arc — this bumps the reference count, both clones point to the same Mutex
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Each thread increments the counter 100 times
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // Lock is released here when `num` goes out of scope
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for every thread to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Lock one more time to read the final value
    let final_value = *counter.lock().unwrap();
    final_value
}
