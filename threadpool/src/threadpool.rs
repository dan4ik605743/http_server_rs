//! ThreadPool structure implementation module

mod worker;

use anyhow::{bail, Result};
use std::sync::{mpsc, Arc, Mutex};
use thiserror::Error;
use worker::Worker;

/// Structure ThreadPool
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will panic if the size is zero.

    pub fn build(size: usize) -> Result<ThreadPool> {
        if size == 0 {
            bail!(ThreadPoolError::ZeroThreads);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..=size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(Self {
            workers,
            sender: Some(sender),
        })
    }
}

impl ThreadPool {
    /// Function accepting work for threads.

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.as_ref().unwrap().send(Box::new(f));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        self.workers.iter_mut().for_each(|worker| {
            println!("Shuttind down worker {}", worker.get_id());

            if let Some(thread) = worker.get_mut_thread().take() {
                thread.join().unwrap();
            }
        });
    }
}

#[derive(Error, Debug)]
enum ThreadPoolError {
    #[error("Trying to create zero threads.")]
    ZeroThreads,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn check_error() {
        let pool = ThreadPool::build(0);
        pool.unwrap();
    }

    #[test]
    fn test() {
        let n_worker = 4;
        let n_jobs = 5;
        let pool = ThreadPool::build(n_worker).unwrap();
        let (tx, rx) = mpsc::channel();

        for _ in 0..n_jobs {
            let tx = tx.clone();
            pool.execute(move || {
                tx.send(1).unwrap();
            });
        }
        assert_eq!(rx.iter().take(n_jobs).sum::<i32>(), n_jobs as i32);
    }
}
