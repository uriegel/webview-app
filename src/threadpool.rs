use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize)->ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers: Vec<_> =
            (0..size)
                .map(|id| Worker::new(id, Arc::clone(&receiver)))
                .collect();

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        println!("New request");
        self.sender.as_ref().unwrap().send(job).unwrap();
    }    
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job(); 
                }
                Err(_) => {
                    println!("Worker {id} disconnecting; shutting down.");
                    break;
                }
            }

        });

        Worker { id, thread: Some(thread) }
    }
}