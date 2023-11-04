use std::{
    thread,
    sync::{mpsc::{self, Receiver}, Mutex, Arc}
};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("[ERROR] THIS LOCK SHOULD NOT BE ACCQUIRED BY OTHER THREADS")
                .recv()
                .map_err(|err| {
                    eprintln!("[ERROR] RECEIVER NOT RECEIVING ANY MESSAGES: {}", err);
                });
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job()
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread) 
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {

        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // How many workers to create
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender) }
    }

    pub fn execute<F>(&self, f:F)
    where
        F:FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread)  = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
