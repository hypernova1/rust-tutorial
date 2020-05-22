/*
    스레드를 이용하여 코드를 동시에 실행
        - 스레드는 프로그램을 복잡하게 만들고 실행 순서에 대한 보장이 없음 이는 다음과 같은 문제들을 야기할 수 있음
            - 경쟁조건(race condition): 여러 스레드들이 일관성 없는 순서로 하나의 데이터나 리소스에 접근하게 되는 것
            - 데드록(deadlock): 두 스레드가 서로 상대 스레드가 가지고 있는 리소스의 사용이 끝나길 기다려서 양쪽 스레드 모두 실행되는 것을 막는 것
            - 특정한 상황에서만 발생하여 재현하기와 안정적으로 수정하기 힘든 버그들

        - 많은 운영 체제들이 새로운 스레드를 만들기 위한 API를 제공
        - 언어가 운영체제의 API를 호출하여 스레드를 만드는 구조는 1:1이라 불림, 하나의 운영 체제 스레드가 하나의 언어 스레드에 대응 된다는 의미

        - 그린 스레드
            - 프로그래밍 언어들 자체가 구현해 놓은 스레드
            - 그린 스레드를 사용하는 언어들은 다른 숫자의 운영 체제 스래드로 구성된 콘텍스트 내에서 그린 스레드를 실행 (M:N 구조)
*/

use std::thread;
use std::time::Duration;

fn main() {
    // thread1();
    // thread2();
    // thread3();
    thread_with_move();

}

// spawn으로 새로운 스레드 생성
fn thread1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
} // 생성된 스레드가 9까지 출력되길 기대했으나 5까지만 출력됨 -> 메인 스레드가 종료되었기 때문

// join을 사용하여 모든 스레드들이 끝날 때까지 기다림
fn thread2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// join을 메인 스레드의 for 루프 이전으로 옮김
fn thread3() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
} // 생성된 스레드가 종료되길 기다린 후 메인 스레드의 for 루프가 실행되기 때문에 교차로 출력되지 않음

// move 클로저 사용
fn thread_with_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // move를 사용하여 v의 소유권을 가져옴
        println!("Here's a vector: {:?}", v);
    });

    // println!("Here's a vector {:?}", v); // 소유권이 이동되어 컴파일 에러
    handle.join().unwrap();
}
// move를 쓰지 않으면 v를 빌리려는 시도를 하지만 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없으므로 v에 대한 참조자가 항상 유효한지 알 수 없음
