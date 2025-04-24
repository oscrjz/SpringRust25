//assignment 1
// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Main thread starting");

//     let mut handles = vec![];

//     for i in 1..=3 {
//         let handle = thread::spawn(move || {
//             println!("Thread {} starting", i);
//             thread::sleep(Duration::from_millis(500));
//             println!("Thread {} finished", i);
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("All threads completed.");
// }

//assignment 2
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _i in 1..=5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
