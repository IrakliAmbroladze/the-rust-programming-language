use std::thread;

pub struct ThreadPool {
    threads: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id));
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
    // --snip--

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| -> () {}),
        }
    }
}
