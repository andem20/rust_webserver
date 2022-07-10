use std::{
    sync::{mpsc, Arc, Mutex},
};

use self::{message::Message, worker::Worker};

mod message;
mod worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: u16) -> ThreadPool {
        let mut workers = Vec::with_capacity(size as usize);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
    
    pub fn get_num_active_threads(&self) -> u16 {
        let workers = &self.workers;

        let mut active_threads: u16 = 0;

        for worker in workers {
            if worker.is_finished() { active_threads += 1; }
        }

        active_threads
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.get_id());

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                worker.set_is_finished(true);
            }
        }
    }
}