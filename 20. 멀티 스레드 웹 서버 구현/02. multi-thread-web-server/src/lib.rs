use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>
}

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

        let mut threads = Vec::with_capacity(size);

        for _ in 1..size {

        }

        ThreadPool {
            threads
        }
    }

    //클로저를 한 번만 실행하기 때문에 FnOnce(), 클로저가 스레드에서 스레드로 이동을 해야하기 때문에 Send, 스레드가 언제 파괴될 지 모르기 때문에 'static
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}