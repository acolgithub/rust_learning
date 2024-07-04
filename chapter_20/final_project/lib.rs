use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// create threadpool struct
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>  // holds sender
}


struct Job;


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero.
    ///
    // create implementation of new threadpool object
    pub fn new(size: usize) -> ThreadPool {

        // check if size is positive and panic otherwise
        assert!(size > 0);

        // extract sender and receiver from channel object
        let (sender, receiver) = mpsc::channel();

        // use Arc to let multiple workers own the receiver and Mutex to
        // ensure only one worker gets a job form the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        // initalize threads vector with capacity matching given size
        let mut workers = Vec::with_capacity(size);

        // loop to create threads
        for id in 0..size {
            // create some threads and store them in the vector
            // workers holding the threads have ids
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // create threadpool vector
        ThreadPool {workers, sender}
    }

    // create new execute implemtation
    // used FnOnce since we intend to pass argument to spawn which uses this
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}


// create struct of workers
// struct is private since only library needs implementation details
struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>
}

// create new worker instance
impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		// spawn new thread for worker
		let thread = thread::spawn(|| {
            receiver;
        });

		// create worker with id
		Worker {id, thread}
	}
}





















