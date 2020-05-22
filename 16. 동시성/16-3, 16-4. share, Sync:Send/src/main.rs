/*
    공유 상태 동시성
    
    Mutex
        - 하나의 스레드만 데이터 접근을 허용
        - 뮤텍스 내부의 데이터에 접근하기 위해서 스레드는 뮤텍스 락을 요청하는 신호를 보내야 함
            -락: 누가 배타적으로 데이터에 접근하는지 추적하는 뮤텍스의 데이터 구조
        - 뮤텍스 두가지 규칙
            - 사용하기 전에 반드시 락을 얻는 시도를 해야함
            - 뮤텍스가 보호하는 데이터의 사용이 끝났다면 다른 스레드들이 락을 얻을 수 있도록 언락해야 함
*/
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // share();
    share2()
    
}

// 단일 스레드에서 Mutex<T>를 사용
fn share() {
    let m = Mutex::new(5);

    {
        // 뮤텍스 내의 데이터에 접근하기 위해 lock메소드 호출, 스레드가 패닉 상태의 락을 가지고 있을 경우를 대비해 unwrap 호출
        let mut num = m.lock().unwrap(); 
        *num = 6; // Mutex<T>는 스마트 포인터
    }

    println!("m = {:?}", m);
}

// Arc를 사용하여 여러 스레드들 사이에서 Mutex<T> 공유
// Arc: 동시적 상황에서 안전하게 사용할 수 있는 Rc<T>
fn share2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Mutex는 데드락이을 생성할 위험이 있음


/*
    Sync와 Send 트레잇

    Sync를 사용하여 여러 스레드로부터의 접근 허용
        - Sync 마커 트레잇은 Sync가 구현된 타입이 여러 스레드로부터 안전하게 참조 가능함을 나타냄
        - &T가 Send이면 T는 Sync함 또한 Sync한 타입들로 구성된 타입도 Sync함

    Send와 Sync를 직접 구현하는 것은 안전하지 않음
*/