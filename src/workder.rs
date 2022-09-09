use crate::util::num_cpus;
use crossbeam_deque::{Stealer, Worker};
use std::any::Any;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Tasks<T, R> {
    worker: Worker<Work<T>>,
    threads: Vec<JoinHandle<Vec<R>>>,
}

#[derive(Debug)]
enum Work<T> {
    Exit,
    Task(T),
}

impl<T, R> Tasks<T, R>
where
    T: FnOnce() -> Option<R>,
    T: Send + 'static,
    R: Send + 'static,
{
    pub fn new() -> Self {
        let cpus = num_cpus();
        let worker = Worker::new_fifo();
        let mut threads = Vec::with_capacity(cpus);

        for _ in 0..cpus {
            let stealer: Stealer<Work<T>> = worker.stealer().clone();
            threads.push(thread::spawn(move || {
                let mut result = Vec::new();
                loop {
                    let work = match stealer.steal().success() {
                        Some(work) => work,
                        None => continue,
                    };

                    match work {
                        Work::Task(task) => match task() {
                            Some(data) => result.push(data),
                            None => continue,
                        },
                        Work::Exit => break,
                    }
                }
                result
            }));
        }

        Self { worker, threads }
    }

    pub fn push(&self, f: T) {
        self.worker.push(Work::Task(f));
    }

    pub fn result(self) -> Vec<Result<Vec<R>, Box<dyn Any + Send + 'static>>> {
        let mut rst = Vec::with_capacity(self.threads.len());
        for _ in 0..self.threads.len() {
            self.worker.push(Work::Exit);
        }
        for thread in self.threads {
            rst.push(thread.join());
        }
        rst
    }
}
