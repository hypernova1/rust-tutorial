/*
    열거형(Enum)
*/

enum IpAddrKind {
    V4,
    V6,
}

struct  IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
    열거형 메소드 정의
*/
impl Message {
    fn call(&self) {
    }
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    /*
        Option

        enum Option<T> {
            Some(T),
            None,
        }
    */
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum =  x + y; // error


}
