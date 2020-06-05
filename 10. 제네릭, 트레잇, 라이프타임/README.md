# 제네릭타입, 트레잇, 라이프타임

[1. 제네릭 타입](#1-제네릭-타입)  
[2. 트레잇](#2-트레잇)  
[3. 라이프타임](#3-라이프타임)

## 중복 코드 다루기

### I. 가장 큰 수를 찾는 코드

~~~rust
fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let mut largest = numbers[0];

  for number in numbers {
    if number > largest {
      largest = number;
    }
  }

  println!("The largest number is {}", largest);
}
~~~

### II. 여러번 사용시

~~~rust
fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let mut largest = numbers[0];

  for number in numbers {
    if number > largest {
        largest = number;
    }
  }

  println!("The largest number is {}", largest);

  let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let mut largest = numbers[0];

  for number in numbers {
    if number > largest {
        largest = number;
    }
  }

  println!("The largest number is {}", largest);
}
~~~

* 중복된 코드가 발생하여 오류가 발생하기 쉽고, 유지보수를 하기가 어려움

### III. 함수로 추출

~~~rust
fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let result = largest(&numbers);
  println!("The largest number is {}", result);
  
  let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let result = largest(&numbers);
  println!("The largest number is {}", result);
}
~~~

## 1. 제네릭 타입

* 구체화된 타입이나 다른 속성들에 대하여 추상화를 하는 대리인

### I. 함수 정의 내에서 제네릭 타입 이용하기

#### 이름과 시그니처만 다른 두 함수

~~~rust
fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
        largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let result = largest_i32(&numbers);
  println!("The largest number is {}", result);

  let chars = vec!['y', 'm', 'a', 'q'];

  let result = largest_char(&chars);
  println!("The largest char is {}", result);
}
~~~

* 같은 작업을 하지만 타입이 달라 함수를 두 번 선언해야함

#### 제네릭 사용

~~~rust
fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let result = largest(&number);
  println!("The largest number is {}", result);

  let chars = vec!['y', 'm', 'a', 'q'];

  let result = largest(&chars);
  println!("The largest char is {}", result);

}
~~~

#### 컴파일시 에러 발생

~~~
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
~~~

* `T`가 될 수 있는 모든 타입에 대해서 동작하지 않는다는 뜻
* `std::cmp::PartialOrd`: 트레잇 타입(비교연산을 가능하게 함)

### II. 구조체 정의 내에서 제네릭 타입 사용

~~~rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
}
~~~

#### 다른 타입 할당시

~~~rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let wont_work = Point { x: 5, y: 4.0 };
}
~~~

~~~
error[E0308]: mismatched types
 -->
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
  floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
~~~

* 동일한 타입을 지정하지 않아서 타입 불일치 에러 발생

#### 두 타입을 이용한 제네릭

~~~rust
stuct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 }; //같은 타입 사용 가능
  let both_integer = Point { x: 1.0, y: 4.0 };
  let both_integer = Point { x: 5, y: 4.0 };
}
~~~

### III. 열거형 정의 내에서 제네릭 사용

#### `Option<T>`

~~~rust
enum Option<T> {
  Some(T),
  None,
}
~~~

#### `Result<T, E>`

~~~rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
~~~

### IV. 메소드 정의 내에서 제네릭 사용

~~~rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };

  println!("p.x = {}", p.x());
}
~~~

~~~rust
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p2 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
~~~

* `mixup`함수의 `V`, `W`는 해당 메소드 내에서만 유효하기 때문에 사용 가능

### V. 제네릭을 이용한 코드의 성능

* 컴파일 타임에 단형성화(*monomorphizaion`)을 수행함으로써 구체적인 타입을 명시했을 때와의 성능 차이가 없음
  * 단형성화: 제네릭 코드를 컴파일 타임에 실제 타입으로 변환

#### 예제

~~~rust
let integer = Some(5);
let float = Some(5.0);
~~~

#### 컴파일 후

~~~rust
enum Option_i32 {
  Some(i32),
  None,
}

enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let integer = Option_i32(5);
  let float = Option_f64(5.0);
}
~~~

## 2. 트레잇

* 공유 동작 정의
* 타 언어의 인터페이스와 유사하지만 몇가지 다른 점이 있음

### I. 정의하기

~~~rust
pub trait Summarizable {
  fn summary() -> String;
}
~~~

### II. 특정 타입에 대한 트레잇 구현

~~~rust
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summarizable for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summarizable for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
~~~

#### 메소드 호출

~~~rust
let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  retweet: false,
}

println!("1 new tweet: {}", tweet.summary());
~~~

#### 다른 스코프에 있는 트레잇 호출

~~~rust
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
  high_temp: f64,
  low_temp: f64,
  chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
  fn summary(&self) -> String {
    format!("The high will be {}, and the low will be {}. The chanch of precipitation is {}%", self.high_temp, self.low_temp, self.chance_of_precipitation)
  }
}
~~~

* `aggregator` 크레이트로부터 다른 크레이트 내의 스코프로 `Summarizable` 트레잇 가져오기
* 트레잇 및 타입이 해당 크레이트 내의 것일 경우에만 해당 타입에서의 트레잇을 정의할 수 있음
* 외부 타입에 대한 외부 트레잇 구현은 허용되지 않음

~~~rust
impl Display for Vec {} //불가능
~~~

#### 기본 구현

~~~rust
pub trait Summarizable {
  fn summary(&self) -> String {
    String::from("(Read more...)")
    //가본 동작을 구현하여 그대로 사용하거나 오버라이딩 하도록 함
  }
}

impl Summarizable for NewsArticle {}

fn main() {
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
  };

  println!("New article available! {}", article.summary()); //(Read more...) 출력
}
~~~

#### 기본 구현이 되어있지 않은 메소드 호출

~~~rust
pub trait Summarizable {
  fn author_summary(&self) -> String;

  fn summary(&self) -> String {
    format!("(Read more from {}...)", self.author_summary())
  }
}

impl Summarizable for Tweet {
  fn author_summary(&self) -> String {
    format!("@{}", self.username)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summary());
}
~~~

### III. 트레잇 바운드

* 제네릭 타입에 제약을 가하고 제네릭이 특정한 트레잇을 구현하여 트레잇이 가지고 있는 동작을 사용할 수 있게 함

~~~rust
pub fn notify<T: Summarizable>(item: T) {
  println!("Breaking news! {}", item.summary());
}

//+를 이용하면 여러개의 트레잇 바운드 특정 가능
fn some_function()<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
  //...
}

//where 절을 이용하여 작성
fn some_function<T, U>(t: T, u: U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
  //...
}
~~~

### IV. 트레잇 바운드를 이용하여 `largest` 함수 고치기

#### 값을 비교하기 위하여 트레잇 바운드 내에 `PartialOrd` 를 특정

~~~rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
  //...
}
~~~

#### 컴파일시

~~~
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
~~~

* `i32`, `char` 같은 고정된 크기의 타입(Copy 트레잇이 구현된)이 아닌 타입들의 소유권을 파라미터로 옮기지 못하여 발생하는 에러

#### 수정

~~~rust
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  let result = largest(&numbers);
  println!("The largest number is {}", result);

  let chars = vec!['y', 'm', 'a', 'q'];

  let result = largest(&chars);
  println!("The largest char is {}", result);
}
~~~

## 3. 라이프타임

* 해당 참조자가 유효한 스코프
* 대부분의 경우에 암묵적이며 추론됨
* 명시해야하는 경우에는 제네릭 라이프타임 파라미터를 이용하여 관계를 명시

### I. 댕글링 참조자 방지

~~~rust
  {
    let r;

    {
      let x = 5;
      r = &x;  
    }
  }

  println!("r {}", r);
~~~

#### 컴파일시

~~~
error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
~~~

* 변수 x는 내부 스코프가 종료되면 drop되기 때문에 r보다 오래 살지 못함
* 따라서 r은 댕글링 참조자가 됨

### II. 빌림 검사기(Borrow checker)

* 모든 빌림이 유효한지 결정하기 위해 스코프를 비교

~~~rust
{
  let r                   // -------+-- 'a
                          //        |
  {                       //        |
    let x = 5;            // -+-----+-- 'b
    r = &x;               //  |
  }                       // -+
                          //  |
  println!("r: {}", r);   //  |  
}                         // -+
~~~

* `x`의 라이프타임인 `'b` 블록은 `r`의 라이프타임인 `'a` 블록보다 훨씬 작기 때문에 컴파일 오류 발생

~~~rust
{
  let x = 5;            // -----+-- 'b
                        //      |
  let r = &x;           // --+--+-- 'a
                        //   |  |
  println!("r: {}", r); //   |  |
                        // --+  |
}                       // -----+
~~~

* `x`의 라이프타임이 `r`의 라이프 타임보다 길기 때문에 `r`의 참조자가 언제나 유효함

### III. 함수에서의 제네릭 라이프타임

#### 두 스트링 슬라이스 중 길이가 긴 스트링 슬라이스를 반환하는 함수

~~~rust
fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string.as_str(), string2);
  println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
~~~

#### 컴파일시

~~~
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
~~~

* 반환되는 참조자가 `x` 를 참조하는 지 `y` 를 참조하는 지 알 수 없기 때문에 참조자를 명시해야함

#### 라이프타임 명시 문법

* 라이프타임 명시는 연관된 참조자의 생명주기를 바꾸는 것이 아님
* 함수의 시그니처가 제네릭 타입 파라미터를 특정할 때라면 어떤 라이프타임을 가진 참조자라도 허용
* 라이프타임을 명시하는 것은 여러개의 참조자에 대한 라이프타임들을 서로 연관짓도록 하는 것

~~~rust
&i32        //참조자
&'a i32     //라이프타임 참조자
&'a mut i32 //가변 라이프타임 참조자
~~~

#### 함수 시그니처 내의 라이프타임 명시

~~~rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
~~~

* 시그니처 내의 모든 참조자들이 동일한 라이프타임 `'a` 가져야한다고 명시
* 라이프타임 명시는 시그니처에만 있으며 함수 내부에선 쓰지 않음

#### 라이프타임이 다른 참조자를 사용하여 함수 호출

~~~rust
fn main() {
  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result = longest(string.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}
~~~

* 두 스트링의 라이프타임이 달라서 컴파일 오류 발생

#### `result`의 선언부를 내부 스코프 밖으로 옮겼을 때

~~~rust
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", result);
}
~~~

#### 컴파일 에러

~~~
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
~~~

* `result`가 `println!`에서 유효하려면 `string2`가 외부 스코프 끝까지 유효해야하는데 내부 블록에서 drop되었기 때문

### IV. 라이프타임의 측면에서 생각하기

#### 라이프타임 생략

~~~rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}
~~~

* `y`는 `x` 혹은 반환 값의 라이프타임과 관련이 없기 때문에 라이프타임 생략 가능

~~~rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}
~~~

#### 컴파일시

~~~
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block
at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
~~~

* `result`는 함수가 종료되면서 drop이 되어 메모리 해제가 발생하기 때문에 댕글링 포인터를 반환
* 반환 타입에 대해 `'a`를 특정했지만 파라미터의 라이프타임과 관련이 없기 때문에 컴파일 에러


### V. 구조체 정의 상에서 라이프타임 명시

#### 참조자를 소유하고 있는 구조체

~~~rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let novel = String::from("Call me Ishmeal. Some years ago...");
  let first_sentence = novel.split(".");
      .next()
      .unwrap();

  let i = ImportantExcerpt { part: first_sentence };
}
~~~

### VI. 라이프타임 생략

~~~rust
fn first_word(s: &str) -> &str {
  let bytes = s.as_str();

  for (i, &item) in bytes.iter().enumerate() {
    if item = b' ' {
      return &s[0..i]
    }
  }

  &s[..]
}
~~~

* 러스트 1.0 이전의 버전에선 위의 코드는 컴파일 되지 않아 아래 처럼 라이프타임을 명시해줘야 했음

~~~rust
fn first_word<'a>(s: &'a str) -> &'a str {
~~~

* 하지만 특정한 상황에서 이러한 코드를 많이 작성하는 것을 알게 되고 그후 빌림 검사기가 라이프타임을 추론할 수 있도록 바뀌었음

#### 라이프타임 생략 규칙

1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 가짐
2. 만약 하나의 라이프타임 파라미터만 있다면 그 라이프타임이 모든 출력 라이프타임 파라미터에게 대입됨
3. 메소드에서 여러개의 입력 라이프타임이 있고 그 중 하나가 `&self` 혹은 `&mut self`라면 `self`라이프타임이 모든 출력 라이프타임 파라미터에 대입됨

### VII. 메소드 정의 내에서의 라이프타임 명시

~~~rust
impl<'a> InportantExcerpt {
  fn level(&self) -> i32 {
    3
  }
}
~~~

* 첫 번째 생략 규칙에 의해 `self`로의 참조자의 라이프타임 명시 생략

~~~rust
impl<'a> ImportantExperpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}
~~~

* 첫 번째 생략 규칙에 의해 `self`와 `announcement`에 각각 라이프타임 부여
* 파라미터중 하나가 `&self`이므로 반환 타입은 `&self`의 라이프타임을 얻음

### VIII. 정적 라이프타임(static lifetime)

* 프로그램의 전체 라이프사이클을 가리킴
* 모든 스트링 리터럴의 라이프타임
* 스트링 리터럴은 프로그램의 바이너리 내에 직접 저장되어 항상 이용 가능
* 아래와 같이 명시적으로 선언 가능

~~~rust
let s: &'static str = "I have a static lifetime";
~~~

### IX. 제네릭 파라미터, 트레잇 바운드, 라이프타임을 함께 쓰기

~~~rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
~~~
