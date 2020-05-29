/*
    싱글 스레레드 기반 웹 서버
        - 주요 프로토콜
            - HTTP: 요청
                - TCP의 상위 기술
                - 요청과 응답의 내용 정의
            - TCP: 응답
                - 저레벨 프로토콜
                - 서버에서 서버로 요청을 보낼 때 사용
                - 해당 정보가 무엇인지 특정하지는 않음
*/
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //80번 포트는 관리자 권한이 필요, 비 관리자는 1024 이상의 포트번호 사용

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //서버와 클라이언트간의 열려있는 커넥션

        println!("Connection established");

        handle_connection3(stream);
    }
}

// 요청 데이터 읽고 HTML 응답
fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap(); //TcpStream으로부터 읽어들인 바이트를 버퍼에 할당

    let contents = fs::read_to_string("hello.html").unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..])); //from::utf8_lossy: &[u8]을 String으로 변환

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents); // \r\n\r\n(CRLF 시퀀스): 요청 라인을 나머지 요청 데이터와 분리

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// 요청을 확인하고 선택적으로 응답
fn handle_connection2(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; //b""를 추가해 바이트 문자열로 변환

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

//리팩토링
fn handle_connection3(mut stream: TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HtTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}