// This chapter is dedicated to the concurrency.

use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// THREADS & JOIN
// ================================================================================================

// ----- 1 --------------------------------------
// Spawn multiple threads to calculate squares of the provided numbers and collect the results.

pub fn calculate_squares(input_numbers: Vec<i32>) -> Vec<i32> {
    let mut handles = vec![];
    let mut results = vec![];

    for number in input_numbers {
        let handle = thread::spawn(move || {
            number * number
        });
        handles.push(handle);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}

// ----- 2 --------------------------------------
// Implement a `parallel_prime_check` function that splits work across multiple threads.

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

/// Inputs:
/// - `numbers` - a `u64` vector of values which should be checked.
/// - `number_of_threads` - a number of threads you must use to *efficiently* distribute the values
///   from the numbers vector.
///
/// Outputs:
/// - `Vec<(u64, bool)>` is a vector of the provided values along with the boolean flag whether this
///   value is prime.
pub fn parallel_prime_check(numbers: Vec<u64>, number_of_threads: usize) -> Vec<(u64, bool)> {
    let num_tasks = numbers.len();
    if num_tasks == 0 {
        return Vec::new();
    }
    
    let numbers_arc = Arc::new(numbers);
    let mut handles = vec![];
    
    let chunk_size = (num_tasks + number_of_threads - 1) / number_of_threads;

    for i in 0..number_of_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, num_tasks);

        if start >= num_tasks {
            break;
        }

        let numbers_clone = Arc::clone(&numbers_arc);

        let handle = thread::spawn(move || {
            let mut chunk_results = Vec::new();
            for &number in &numbers_clone[start..end] {
                chunk_results.push((number, is_prime(number)));
            }
            chunk_results
        });
        handles.push(handle);
    }
    
    let mut results = vec![];

    for handle in handles {
        match handle.join() {
            Ok(chunk_results) => results.extend(chunk_results),
            Err(_) => eprintln!("A thread panicked."),
        }
    }

    results
}

// MPSC CHANNELS
// ================================================================================================

// ----- 3 --------------------------------------
// Compute the factorial for each value in the provided vector.
// Use a separate thread for each computation.
// Send the factorial results to the main thread using a channel transmitter.
// Using a channel receiver, collect the resulting factorial values into a vector and return it from
// the function.

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn parallel_factorials(numbers: Vec<u32>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    let total_tasks = numbers.len();

    for number in numbers {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = factorial(number);
            tx_clone.send(result).unwrap();
        });
        handles.push(handle);
    }

    drop(tx); 

    rx.into_iter().collect()
}

// MUTEX + ARC
// ================================================================================================

// ----- 4 --------------------------------------
// Implement a `SharedCounter` struct with one `value: ?<i32>` field and methods:
// - `pub fn new(initial_value: i32) -> Self`, which creates a new instance of the `SharedCounter`.
// - `pub fn increment(&self)` which will increment the internal value.
// - `pub fn get_value(&self) -> i32` which will return the internal value.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial_value: i32) -> Self {
        SharedCounter {
            value: Arc::new(Mutex::new(initial_value)),
        }
    }

    pub fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    pub fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

// ----- 5 --------------------------------------
// Simulate a bank account system with concurrent deposits and withdrawals.
//
// Implement a `BankAccount` struct with one `balance: ?<i32>` field and methods:
// - `pub fn new(initial_balance: i32) -> Self`, which creates a new instance of the `BankAccount`.
// - `pub fn deposit(&self, amount: i32)` which adds the provided amount to the balance.
// - `pub fn withdraw(&self, amount: i32) -> bool` which attempts to remove the provided amount from
//   the balance. If the balance have sufficient funds, it removes the provided amount and returns
//   `true`, otherwise returns `false`.
// - `pub fn get_balance(&self)` which returns the current balance.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct BankAccount {
    balance: Arc<Mutex<i32>>,
}

impl BankAccount {
    pub fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    pub fn deposit(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
    }

    pub fn withdraw(&self, amount: i32) -> bool {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_balance(&self) -> i32 {
        *self.balance.lock().unwrap()
    }
}

// FINAL BOSS: CHANNELS + MUTEX + ARC
// ================================================================================================

// ----- 6 --------------------------------------
// Implement a work queue where multiple workers consume tasks and send results back (a simple task
// distribution system).
//
// You will need to implement two procedures:
// - `worker(id: usize, task_receiver: ?<Receiver<i32>>, result_sender: ?<Sender<(usize, i32)>>)`,
//   which has the ID of the worker, the task receiver, which waits for the task to be provided to
//   this worker, and the result sender, which sends the computed result back to the main thread.
//   This procedure should:
//   - Loop over all incoming tasks from `task_receiver`.
//   - For each task, compute the square of the value this task provided.
//   - Send the result back via `result_sender` along with the worker’s ID:
//    `(worker_id, result_value)`.
//   - Decide by your own whether this procedure should return something or not, and if should --
//    what exactly.
//   - Use `Arc` or `Mutex` if needed.
// - `run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)>` which has the
//   vector of tasks (just values, which square we should compute) and the total number of workers
//   which should be spawned. It returns the vector of worker IDs (`usize`) and the resulting value
//   computed by this worker (`i32`). This procedure should:
//   - Create two channels: for sending tasks to workers and for collecting results from workers.
//   - For each worker spawn a thread which runs the worker function, consuming tasks and sending
//     results.
//   - Send each task from the input list into the task_sender.
//   - Collect all results from the result_receiver into a vector and return it.

fn worker(worker_id: usize, task_receiver: Arc<Mutex<Receiver<i32>>>, result_sender: Sender<(usize, i32)>) {
    loop {
        let task = {
            let locked_receiver = task_receiver.lock().unwrap();
            locked_receiver.recv()
        };

        match task {
            Ok(num) => {
                let result = num * num;
                if result_sender.send((worker_id, result)).is_err() {
                    break;
                }
            },
            Err(_) => break,
        }
    }
}

pub fn run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)> {
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let shared_task_rx = Arc::new(Mutex::new(task_rx));

    let mut handles = vec![];

    for i in 0..number_of_workers {
        let task_receiver_clone = Arc::clone(&shared_task_rx);
        let result_sender_clone = result_tx.clone();
        
        let handle = thread::spawn(move || {
            worker(i, task_receiver_clone, result_sender_clone);
        });
        handles.push(handle);
    }
    
    drop(result_tx);

    for task in tasks.into_iter() {
        if task_tx.send(task).is_err() {
            break;
        }
    }

    drop(task_tx);

    let mut results: Vec<(usize, i32)> = result_rx.into_iter().collect();

    for handle in handles {
        handle.join().unwrap();
    }
    
    results
}