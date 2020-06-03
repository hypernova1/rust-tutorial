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
