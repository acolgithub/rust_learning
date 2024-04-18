// use statements
use std::thread;
use std::time::Duration;


fn main() {
    // create a new thread and pass closure containing code we want to run in the new thread
    thread::spawn(|| {
        for i in 1..16 {
            println!("hi number {} from the spawned thread!", i);  // print a number
            thread::sleep(Duration::from_millis(1))  // sleep for a millisecond, this allows different thread to run
        }
    });


    // create another new thread that we call the join method on ensuring the thread will finish
    let handle = thread::spawn(|| {  // assigning return value (JoinHandle) to variable
        for i in 1..8 {
            println!("hi number {} from the second spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });


    // create a third thread that we call the join method on before the main handle so it should cause the main handle to wait for the third thread to finish before starting
    let handle2 = thread::spawn(|| {  // assigning return value (JoinHandle) to variable
        for i in 1..6 {
            println!("hi number {} from the third spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // call to join
    handle2.join().unwrap();

    // create a vector
    let v = vec![1, 2, 3];

    // create a final thread which will move vector
    let handle_final = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });


    // main thread not guaranteed to execute at specific time in relation to created thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));  // sleep for a millisecond, this allows different thread to run
    }
    // when main thread completed all threads ended even if not complete

    // apply join method to ensure second thread completes before main thread terminates
    handle.join().unwrap();

    // apply join method to wait for final_handle
    handle_final.join().unwrap();
}
