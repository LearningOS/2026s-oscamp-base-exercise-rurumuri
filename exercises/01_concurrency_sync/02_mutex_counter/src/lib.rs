//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, Thread};

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    // TODO: Spawn n_threads threads
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: Join all threads, return final value
    // todo!()
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..n_threads {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            for _ in 0..count_per_thread {
                let mut num = sum_clone.lock().unwrap();
                *num += 1; 
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_sum = sum.lock().unwrap();
    *final_sum
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return
    // todo!()
    let result = Arc::new(Mutex::new(Vec::<usize>::new()));

    // Bad Implemetation
    // for i in 0..n_threads {
    //     thread::spawn({
    //         let result_clone = Arc::clone(&result);
    //         move || {
    //             let mut vec = result_clone.lock().unwrap();
    //             vec.push(i);
    //         }
    //     }).join().unwrap();
    // }

    let mut handles = vec![];
    for i in 0..n_threads {
        let handle = thread::spawn({
            let result_clone = Arc::clone(&result);
            move || {
                let mut vec = result_clone.lock().unwrap();
                vec.push(i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let mut final_result = result.lock().unwrap();
    final_result.sort();
    final_result.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
