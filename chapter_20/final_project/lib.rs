use std::thread;

// create threadpool struct
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>
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
        let mut threads = Vec::with_capacity(size);

        // loop to create threads
        for _ in 0..size {
            // create some threads and store them in the vector
        }

        // create threadpool vector
        ThreadPool {threads}
    }

    // create new execute implemtation
    // used FnOnce since we intend to pass argument to spawn which uses this
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
























