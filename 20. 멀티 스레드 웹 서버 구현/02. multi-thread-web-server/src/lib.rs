use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate messge to all workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap(); //각 Worker 에 Terminate 메세지 전달
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() { //take: Some 을 None 으로 대체함
                thread.join().unwrap();
            }
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// 새 스레드풀 생성
    ///
    /// size 는 풀 안의 스레드 개수
    ///
    /// #Panics
    ///
    /// `new` 함수는 size 가 0일 때 패닉
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    //클로저를 한 번만 실행하기 때문에 FnOnce(), 클로저가 스레드에서 스레드로 이동을 해야하기 때문에 Send, 스레드가 언제 drop 될 지 모르기 때문에 'static
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/*
    스레드를 생성하고 코드를 전달 받을 때 까지 기다리도록 해야하는데,
    표준 라이브러리의 스레드 구현에는 이런 방법을 지원하지 않아 직접 Worker 로 구현
*/
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} was told to executing", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}