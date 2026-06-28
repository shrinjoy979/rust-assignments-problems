/*
  Problem 100: Async Task Scheduler

  Implement a TaskScheduler that can schedule async closures to run after
  a specific delay. It should use a PriorityQueue (or sorted Vec) to keep track
  of tasks and a background tokio task to execute them when their time comes.

  Run the tests for this problem with:
    cargo test --test task_scheduler_test
*/

use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};

pub struct TaskScheduler {
    pub tasks: Arc<Mutex<Vec<(Instant, Box<dyn FnOnce() + Send + 'static>)>>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn schedule<F>(&self, delay: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let run_at = Instant::now() + delay;

        let mut tasks = self.tasks.lock().unwrap();
        tasks.push((run_at, Box::new(f)));

        // Keep tasks sorted by execution time
        tasks.sort_by_key(|(instant, _)| *instant);
    }

    pub fn start(&self) {
        let tasks = Arc::clone(&self.tasks);

        tokio::spawn(async move {
            loop {
                let next_task = {
                    let mut queue = tasks.lock().unwrap();

                    if queue.is_empty() {
                        None
                    } else if queue[0].0 <= Instant::now() {
                        Some(queue.remove(0))
                    } else {
                        None
                    }
                };

                if let Some((_, task)) = next_task {
                    task();
                } else {
                    // Avoid busy waiting
                    sleep(Duration::from_millis(1)).await;
                }
            }
        });
    }
}
