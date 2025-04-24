// assignment 3
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// type Job = Box<dyn FnOnce() + Send + 'static>;

// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));
//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         for worker in &mut self.workers {
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let message = receiver.lock().unwrap().recv().unwrap();

//             match message {
//                 Message::NewJob(job) => {
//                     println!("Worker {} got a job; executing.", id);
//                     job();
//                 }
//                 Message::Terminate => {
//                     println!("Worker {} was told to terminate.", id);
//                     break;
//                 }
//             }
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// fn main() {
//     let pool = ThreadPool::new(4);

//     for i in 1..=10 {
//         pool.execute(move || {
//             println!("Processing task {}", i);
//             thread::sleep(std::time::Duration::from_millis(500));
//             println!("Completed task {}", i);
//         });
//     }

//     println!("Main thread waiting for tasks to complete...");
// }

//assignment 4
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let count = ITEM_COUNT / NUM_PRODUCERS;
        let handle = thread::spawn(move || {
            producer(i, tx_clone, count);
        });
        handles.push(handle);
    }

    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        handles.push(handle);
    }

    for handle in handles.drain(..NUM_PRODUCERS) {
        handle.join().unwrap();
    }

    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

 
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let value = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished", id);
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = {
            let lock = rx.lock().unwrap();
            lock.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }

        println!("Consumer {} processed value {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
    println!("Consumer {} exiting", id);
}