use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize,
}

impl Worker {
    pub fn get_mut_thread(&mut self) -> &mut Option<thread::JoinHandle<()>> {
        &mut self.thread
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<super::Job>>>) -> Self {
        Self {
            thread: Some(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();

                if let Ok(job) = job {
                    println!("Worker {id} got a job executing.");

                    job();
                } else {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            })),
            id,
        }
    }
}
