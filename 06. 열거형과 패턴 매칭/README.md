# 열거형과 패턴 매칭

[1. 열거형 정의하기](#1-열거형-정의하기)  
[2. match 흐름 제어 연산자](#2-match-흐름-제어-연산자)  
[3. it let을 사용한 간결한 흐름 제어](#3-if-let을-사용한-간결한-흐름-제어)  

## 1. 열거형 정의하기
### I. 정의하기
~~~rust
enum IpAddrKind {
  V4,
  V6,
}
~~~
### II. 열거형 값
~~~rust
let four = IpAddrKind::A4
let six = IpAddrKind::A6
~~~
### III. 사용하기
~~~rust
enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpaddrKind::V4,
  address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
}
~~~
* `struct`를 사용하여 IP주소와 타입 정의하기

#### 변수에 데이터를 직접 넣기 (1)
~~~rust
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
~~~

#### 변수에 데이터를 직접 넣기 (2)
~~~rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
~~~

### III. 여러 데이터 타입 정의하기
~~~rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
~~~

### IV. 메소드 정의하기
~~~rust
impl Message {
  fn call(&self) { //self => "hello"
    //...
  }
}

let m = Message::Write(String::from("hello"));
m.call();
~~~

### V. `Option`
#### V.i. 구조
~~~rust
enum Option<T> {
  Some(T),
  None,
}
~~~

#### V.ii. `Some`과 `None`
~~~rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
~~~

#### V.iii. 사용법
~~~rust
let x: Option<u32> = Some(2);
assert_eq!(x.contains(&2), true);

let x = x.unwrap(); //2
// let x = x.unwrap_or(4)
~~~

## 2. `match` 흐름 제어 연산자
### I. 예제
* 갈래 사용
~~~rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
~~~
### II. 예제2
* `match` 갈래 안에서 또 갈래를 쓸 수 있음
~~~rust
fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
~~~

### III. 값을 바인딩하는 패턴들
* `Quarter`가 `UsState`의 값을 들고 있음
~~~rust
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  //...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}
~~~

### IV. `Option<T>`를 이용하는 매칭
~~~rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
~~~

### V. `match` 규칙
~~~rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
  }
}
~~~
* `match`는 하나도 빠트리면 안됨
* 위의 예제의 경우 `None`을 다루지 않았기 때문에 컴파일 에러

### VI. `_`변경자(placeholder)
* `else`키워드와 같은 개념
* 나머지 패턴들을 모두 체크
~~~rust
let some_u8_value = 0u8;
match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  5 => println!("five"),
  7 => println!("seven"),
  _ => (), // (): 아무일도 일어나지 않음
}
~~~

## 3. `if let`을 사용한 간결한 흐름 제어
### I. `if let`
* 기존 코드
~~~rust
let some_u8_value = Some(0u8);
match some_u8_value {
  Some(3) => println!("three),
  _ => (),
}
~~~
* `if let` 사용
~~~rust
if let Some(3) = some_u8_value {
  println!("three");
}
~~~

### II. `if let`과 `else`
* 기존 코드
~~~rust
let mut count = 0;
match coin {
  Coin::Quarter(state) => println!("State quarter from {:?}", state),
  _ => count += 1,
}
~~~
* `if let` `else`를 사용
~~~rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("State quarter from {:?}", state);
} else {
  count += 1;
}
~~~