/*
    메시지 패싱을 사용하여 스레드 간 데이터 전송
        - 안전한 동시성을 보장
        - 스레드들 혹은 액터들이 데이터를 담고 있는 메세지를 서로 주고 받는 것
        - 두개의 채널을 사용하여 송수신 (송신자/수신자)
*/
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    
    // thread();
    // thread2();
    // thread3();
    thread4();

    
}

fn thread() {
    // transmitter, receiver 가져오기
    let (tx, rx) = mpsc::channel();

    // move를 사용하여 tx를 클로저 안으로 이동시켜 스레드가 소유하게 함
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //tx.send는 Result<T, E>를 반환
    });

    let received = rx.recv().unwrap(); // rx.recv 메인 스레드 실행을 블록시키고 채널로부터 값이 보내질 때까지 기다림
    println!("Got: {}", received);
}

fn thread2() {
    // 채널과 소유권 전달
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); //error! send 함수가 val의 소유권을 가져가고 값이 이동될 때 수신자가 소유권을 얻음
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 여러 메세지를 보내고 각 사이마다 멈춤
fn thread3() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// 송신자를 복제하여 여러 생성자 만들기 (동일한 수신자로 값들을 보냄)
fn thread4() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx); // clone을 호출함으로써 첫번째 생성된 스레드로 값을 보낼 수 있는 송신 핸들을 제공받음
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}