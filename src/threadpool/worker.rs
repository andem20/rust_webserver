use std::{thread, sync::{mpsc, Mutex, Arc}};

use super::message::Message;

pub struct Worker {
    id: u16,
    pub thread: Option<thread::JoinHandle<()>>,
    is_finished: bool
}

impl Worker {
    pub fn new(id: u16, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().expect("Thread Panicked").recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
            is_finished: false
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn set_is_finished(&mut self, is_finished: bool) {
        self.is_finished = is_finished;
    }
}