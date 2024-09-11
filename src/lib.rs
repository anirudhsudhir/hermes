use std::{
    sync::mpsc::{self, Receiver, Sender},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue_tx: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> ThreadPool {
        assert_ne!(worker_count, 0);

        let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(worker_count);

        for id in 0..worker_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            queue_tx: Some(tx),
        }
    }

    pub fn spawn<J>(&self, job: J)
    where
        J: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.queue_tx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.queue_tx.take());

        for worker in &mut self.workers {
            println!("Waiting for thread {} to finish", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    println!("Stopping thread {id}");
                }
            };
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
