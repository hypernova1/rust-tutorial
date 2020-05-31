/*
    서버를 싱글 스레드에서 멀티 스레드로 바꾸기
*/
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //80번 포트는 관리자 권한이 필요, 비 관리자는 1024 이상의 포트번호 사용
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap(); //서버와 클라이언트간의 열려있는 커넥션
//
//         println!("Connection established");
//
//         handle_connection(stream);
//     }
// }

/*
    작업 시간이 오래 걸리는 요청 시뮬레이팅
        - /sleep 요청을 하였을 시 5초가 걸린다 가정하고 5초간 멈춤
*/
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/*
    스레드 풀을 이용한 처리량 증진

    스레드풀
        - 대기중이거나 작업을 처리할 준비가 되어있는 스레드 그룹
        - 여러 커넥션들을 동시에 처리할 수 있게 하고 서버의 처리량을 증가시킴

    스레드풀 동작 방식
        - 프로그램이 새 작업을 받았을 때, 스레드 풀은 작업을 풀 안에 있는 스레드 중에 하나에게 맡기고 해당 스레드가 작업을 처리하도록 함
        - 남은 스레드들은 첫 번째 스레드가 처리중인 동안 대기하여 작업이 들어오면 처리
        - 첫 번째 스레드가 작업을 마치면 풀로 돌아와 작업 대기상태가 됨

    스레드 풀 내의 스레드 개수 제한
        - Dos(Denial of Service) 공격을 막기 위해 스레드 풀 안의 스레드 개수를 작게둠
        - 각각의 요청마다 스레드를 생성한다면 누군가 10만개의 요청을 보냈을 때 서버는 서버의 모든 리소스를 사용하고 모든 요청이 끝날 때까지 처리가 계속 될 것임
        - 따라서 고정된 개수의 스레드를 갖게함
        - 요청이 들어올 시 요청들은 처리를 위해 풀로 보내지고, 풀로 들어온 요청들에 대한 큐를 유지
        - 풀 내의 각 스레드들은 이 큐에서 요청을 꺼내서 처리하고 또 다른 요청이 있는지 큐에 확인
        - 위와 같은 형태를 통해 N개의 요청을 처리 할 수 있음(N은 스레드의 개수)
*/

extern crate hello;
use hello::ThreadPool;

// 스레드 풀 생성
fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
