// create threadpool struct
pub struct ThreadPool;


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

        ThreadPool
    }

    // create new execute implemtation
    // used FnOnce since we intend to pass argument to spawn which uses this
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
























