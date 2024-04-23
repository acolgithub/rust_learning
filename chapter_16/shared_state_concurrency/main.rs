// use statements
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;  // concurrent safe Rc type


fn main() {
    // create new mutex object containing a number
    let m = Mutex::new(5);

    {
        // give access to data held by mutex to num using lock method to block current thread
        let mut num = m.lock().unwrap();  // lock removes LockResult and leaves MutexGuard smart pointer

        // change value num points to
        *num = 6;
    }// smart pointer releases lock when it goes out of scope

    println!("m = {:?}", m);


    let counter = Arc::new(Mutex::new(0));  // wrapped in Rc to allow for multiple owners
    let mut handles = vec![];

    for _ in 0..10 {

        // create clone of counter before moving ownership
        let counter = Arc::clone(&counter);

        // create multiple threads to increment mutex data
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // get access to mutex data

            *num += 1;  // increment mutex data by 1
        });

        handles.push(handle);  // add to handles vector
    }

    for handle in handles {
        handle.join().unwrap();  // call join to make sure all threads finish
    }

    println!("Result: {}", *counter.lock().unwrap());

}
