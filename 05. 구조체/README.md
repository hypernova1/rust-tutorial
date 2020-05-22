# 구조체

[1. 구조체를 정의하고 생성하기](#1-구조체를-정의하고-생성하기)  
[2. 구조체를 이용한 예제 프로그램](#2-구조체를-이용한-예제-프로그램)  
[3. 메소드 문법](#3-메소드-문법)  

## 1. 구조체를 정의하고 생성하기
### I. 구조체 정의
~~~rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
~~~
* 구조체의 구성요소들은 각각 다른 타입을 가질 수 있음

### II. 구조체의 인스턴스 생성
~~~rust
let user1 = User {
  email: String::from("hypemova@gmail.com"),
  username: String::from("sam chan"),
  active: true,
  sign_in_count: 1,
}
~~~
* 프로퍼티 읽기: `.` 사용 ex) `user1.email;`
* 프로퍼티 변경: `.` 사용 ex) `user1.email = String::from("chtlstjd01@naver.com");`

### III. 구조체 갱신법(`..`)
~~~rust
let user2 = User {
  email: String::from("another@gmail.com"),
  username: String::from("another"),
  ..user1
}
~~~

### IV. 튜플 구조제
* 이름이 없고 필드마다 타입이 다름
~~~rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
~~~

> ### 구조체 데이터의 소유권
> * 구조체가 소유권이 없는 데이터의 참조를 저장할 수 있지만 라이프타임의 사용을 전제로함
> * 라이프타임을 사용하지 않고 저장하면 아래와 같은 일이 발생
> ~~~rust
> struct User {
>   username: &str,
>   email: &str,
>   sign_in_count: u64,
>   active: bool,
> }
> 
> fn main() {
>   let user1 = User {
>     email: "hypemova@gmail.com",
>     username: "sam chan",
>     active: true,
>     sign_in_count: 1,
>   };
> 
> }
> ~~~
> ~~~
> error[E0106]: missing lifetime specifier
>  -->
>  |
>2 |     username: &str,
>  |               ^ expected lifetime parameter
>
>error[E0106]: missing lifetime specifier
> -->
>  |
>3 |     email: &str,
>  |            ^ expected lifetime parameter
> ~~~
> * 10 장에서 설명함

## 2. 구조체를 이용한 예제 프로그램

## 3. 메소드 문법
* 메소드는 함수와 유사
* `fn` 키워드로 선언
* 함수와 달리 구조체의 내용 안에 정의
* 첫번째 파라미터는 언제나 `self`
  * `self`: 메소드가 호줄되고 있는 구조체의 인스턴스를 가리킴

### I. 메소드 정의
~~~rust
#[derive(Debug)]
struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  fn area(&self) -> u32 { //인스턴스가 변하길 원한다면 &mut self를 사용
    self.length * self.width
  }
}

fn main() {
  let rect1 = Rectangle { length: 50, width: 30 };

  println!("{}", rect1.area()); //1500
}
~~~

> C++같은 언어들과의 차이점 (`->` 연산자)
> * C++ 과 같은 언어
>   * 메소드 호출을 위해서 두 가지의 연산자가 사용됨
>   * 어떤 객체의 메소드를 직접호출: `.`
>   * 어떤 객체의 포인터에서의 메소드를 호출하는 중이고 이 포인터를 역참조할 필요가 있다면: `->`
>   * ex) `object`가 포인터라면 `object->somthing()`와 `(*object).somthing()`은 비슷함
> * 러스트
>   * `->`와 같은 연산자가 없고 자동 참조 및 역참조(*automatic referencing and dereferencing*)라는 기능이 존재
>    * `object.something()`을 호출하면 자동적으로 `&`, `&mut` 혹은 `*`을 붙여서 `object`가 해당 메소드의 시그니처와 맞도록 함
>   * `p1.distance(&p2)`와 `(&p).distance(&p2)`는 같은 표현
>   * 자동 참조 동작은 메소드가 명확한 수신자(`self`)를 가지고 있기 때문임

### II. 더 많은 파라미터를 가진 메소드
~~~rust

impl Rectangle {
  fn area(&self) -> u32 {
    self.length * self.width
  }
  
  fn can_hold(&self, other: Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}

fn main() {
  let rect1 = Rectangle { length: 50, width: 30 };
  let rect2 = Rectangle { length: 40, width: 10 };
  let rect3 = Rectangle { length: 45, width: 60 };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
~~~

### III. 연관 함수
* `self`파라미터를 갖지 않는 함수
* 주로 구조체의 인스턴스를 반환하는 생성자로 사용됨

~~~rust
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { length: size, width: size }
  }
}

main() {
  let square1 = Rectangle::square(3);
}
~~~