/*
  Problem 71: Multithreaded Sum

  Write a function that takes a Vec<i32> and splits it into two halves.
  Sum each half in a separate thread using std::thread::spawn and return
  the total sum.

  Run the tests for this problem with:
    cargo test --test threaded_sum_test
*/

use std::thread;

pub fn threaded_sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();

    let left_handle = thread::spawn(move || left.iter().sum::<i32>());
    let right_handle = thread::spawn(move || right.iter().sum::<i32>());

    left_handle.join().unwrap() + right_handle.join().unwrap()
}
