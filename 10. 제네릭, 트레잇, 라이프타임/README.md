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
