use std::thread;
use std::time::Duration;

// fn main() {
//     // Returns joinhandle
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     // main shutdown
// }

fn main() {
    let v = vec![1, 2, 3];
    // Rust can’t tell how long the spawned thread will run,
    // so it doesn’t know if the reference to v will always be valid.
    // use move method
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Terrible!
    //drop(v);

    handle.join().unwrap();
}