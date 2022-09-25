use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: replace thread::spawn with https://doc.rust-lang.org/std/thread/struct.Builder.html
        let thread = thread::spawn(move || loop {
            // TODO: Handler errors
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}

#[derive(Debug, PartialEq)]
pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool. If the size is
    /// zero a [`PoolCreationError`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_server::ThreadPool;
    /// let pool = ThreadPool::new(0);
    ///
    /// assert_eq!(pool.is_ok(), false);
    /// ```
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        Ok(ThreadPool { workers, sender })
    }

    // TODO:
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // TODO: Handle error
        self.sender.send(job).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_thread_pool_with_zero_threads() {
        let result = ThreadPool::new(0);
        assert_eq!(result.err(), Some(PoolCreationError));
    }

    #[test]
    fn build_thread_pool_with_two_threads() {
        let result = ThreadPool::new(2);
        assert_eq!(result.is_ok(), true);
        if let Ok(pool) = result {
            assert_eq!(pool.workers.len(), 2);
            assert_eq!(pool.sender.is_some(), true);
        }
    }
}
