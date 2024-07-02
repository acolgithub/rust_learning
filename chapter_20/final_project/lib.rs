use std::thread;

// create threadpool struct
pub struct ThreadPool {
    workers: Vec<Worker>
}


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

        // initalize threads vector with capacity matching given size
        let mut workers = Vec::with_capacity(size);

        // loop to create threads
        for id in 0..size {
            // create some threads and store them in the vector
	    // workers holding the threads have ids
	    workers.push(Worker::new(id));
        }

        // create threadpool vector
        ThreadPool {workers}
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
	fn new(id: usize) -> Worker {
		// spawn new thread for worker
		let thread = thread::spawn(|| {});

		// create worker with id
		Worker {id, thread}
	}
}





















