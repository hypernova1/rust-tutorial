# 모듈

[1. mod와 파일 시스템](#1-mod와-파일-시스템)  
[2. pub으로 가시성 제어하기](#2-pub으로-가시성-제어하기)  
[3. use로 이름 가져오기](#3-use로-이름-가져오기)  

## 1. `mod`와 파일 시스템

### I. 라이브러리 크레이트 만들기

~~~
$ cargo new communicator --lib
$ cd communicator
~~~

### II. 내부 코드

Filename: *src/lib.rs*

~~~rust
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
~~~

* main.rs 파일이 없기 때문에 `cargo run`로 실행할 것이 없음
* `cargo build`를 사용


### III. 모듈 정의

Filename: *src/lib.rs*
~~~rust
mod network {
  fn connect() {
  }
}
~~~

* `connect`에 접근하려면 `::`문법을 사용해야함
  * ex) `network::connect()`

~~~rust
mod network {
  fn connect() {
  }
}

mod client {
  fn connect() {
  }
}
~~~

### IV. 모듈 내부에 모듈 정의

~~~rust
mod network {
  fn connect() {
  }

  mod client {
    fn connect() {
    }
  }
}
~~~

### V. 모듈을 다른 파일로 옮기가

* IV의 에제는 시스템이 복잡해질수록 유지보수하기가 힘들기 때문에 따로 폴더 구조를 만듬  

#### 폴더 구조

~~~
communicator
 ├── client
 └── network
     └── server
~~~

Filename: *src/lib.rs*

~~~rust
mod client; // src/client.rs 에서 구현

mod network {
  fn connect() {
  }

  mod server {
    fn connect() {
    }
  }
}
~~~

Filename: *src/client.rs*

~~~rust
fn connect() {
}
~~~

* src/lib.rs에 `client` 모듈을 미리 선언했기 때문에 `mod`선언이 필요 없음

### VI. `network` 모듈 개별 파일로 추출하기

Filename: *src/lib.rs*

~~~rust
mod client;

mod network;
~~~

Filename: *src/network.rs*

~~~rust
fn connect() {
}

mod server {
  fn connect() {
  }
}
~~~

### VII. `server` 모듈 추출하기

Filename: *src/network.rs*

~~~rust
fn connect() {
}
mod server;
~~~

Filename: *src/server.rs*

~~~rust
fn connect() {
}
~~~

~~~
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^
~~~

* `cargo build` 실행시 에러 발생: `server`서브 모듈을 *src/server.rs* 로 추출을 시도했을 때 발생하는 에러

### VIII. 에러 해결

1. network 디렉토리를 생성해서 그 안에 *src/network* 파일 옮기기
2. *src/network/mod.rs* 생성

#### 폴더구조

~~~
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
~~~

### 모듈 파일 시스템의 규칙

* `foo`라는 이름의 서브 모듈을 가지고 있다면 *foo.rs*라는 이름의 파일 내에 `foo`에 대한 선언이 있어야함
* `foo`가 서브모듈을 가지고 있다면 *foo/mod.rs*라는 이름의 파일에 `foo`에 대한 선언이 있어야함

~~~
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
~~~

## 2. `pub`으로 가시성 제어하기

### I. 바이너리 크레이트 만들기

Filename: *communicator/src/main.rs*

~~~rust
extern crate communicator;

fn main() {
  communicator::client::connect();
}
~~~

* `extern` 라이브러리 크레이트를 가져오기 위한 명령어
* *src/main.rs* 파일은 바이너리 크레이트의 루트 파일로 취급 (기존의 라이브러리 크레이트와는 별개)
* 크레이트의 최상위 모델을 루트 모듈이라 부름

### 빌드시 나타나는 오류

~~~
$ cargo build

error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 
~~~

* `client` 모듈이 비공개라서 접근이 불가능

### II. `pub` 키워드를 사용해서 함수를 공개로 만들기

Filename: *src/lib.rs*

~~~rust
pub mod client;

mod network;
~~~

~~~
$ cargo build
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
~~~

* `client` 모듈을 공개로 만들었지만 `connect` 함수가 비공개라서 컴파일 실패
* 수정 코드

Filename: *src/client.rs*

~~~rust
pub fn connect() {
}
~~~

### `cargo build`시 생기는 경고

~~~
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
~~~

* 코드가 컴파일 되었지만 비공개 함수들이 내부에서 사용되지 않았기 때문에 경고가 생김
* `pub`키워드 붙이기  

Filename: *src/network/mod.rs*

~~~rust
pub fn connect() {
}

mod server;
~~~

~~~
$ cargo build

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
~~~

* 여전히 경고가 남아있음
* 함수가 모듈내에서는 공개지만 함수가 상주해있는 `network` 모듈은 공개가 아니기 때문

### `network` 모듈 공개

Filename: *src/lib.rs*

~~~rust
pub mod client;
pub mod network;
~~~

### 비공개 규칙

1. 어떤 아이템이 공개라면, 부모의 모듈 어디서든지 접근 가능
2. 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능

#### 예제

Filename: *src/lib.rs*

~~~rust
mod outermost {
  pub fn middle_function() {}

  fn middle_secret_function() {}

  mod inside {
    pub fn inner_function() {}

    fn secret_function() {}
  }

  fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();

  }
}
~~~

* `try_me`함수는 루트 모듈내에 있음
  * `outermost::middle_function()`: 2번째 규칙에 따라 작동
  * `outermost::middle_secret_function()`: 두번째 규칙으로 인해 컴파일 에러
  * `inside`: 비공개이고 자식 모듈이 없으므로 현재 모듈인 `outermost`에 의해서만 접근 가능 => 컴파일 에러

## 3. use로 이름 가져오기

### use를 사용하지 않았을 때

~~~rust
pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {}
    }
  }
}

fn main() {
  a::series::of::nested_modules();
}
~~~

* `of`모듈 내의 함수를 여러번 호출하려면 항상 긴 코드를 작성해야 함

### I. `use` 사용하기

~~~rust

use a::series::of;

fn main() {
  of::nested_modules();
}
~~~

~~~rust
use a::series::of::nested_modules;

fn main() {
  nested_modules();
}
~~~

* 모듈 내의 함수를 가져오는 것도 가능

#### 열거형 가져오기

~~~rust
enum TrafficLight {
  Red,
  Yellow,
  Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
  let red = Red;
  let yellow = Yellow;
  let greed = TrafficLight::Green;
}
~~~

#### `*`을 이용한 모두(glob) 가져오기

~~~rust
use TrafficLight::*;

fn main() {
  let red = Red;
  let yellow = Yellow;
  let greed = Green;
}
~~~

### II. `super`를 사용하여 부모 모듈에 접근하기

* 라이브러리 크레이트를 생성하면 `tests` 모듈이 생성되어 있음  

Filename: *src/lib.rs*

~~~rust
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
~~~

#### 모듈의 계층 구조

~~~
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
~~~

#### `tests`의 `it_works`함수에서 `client::connect` 호출하기

Filename: *src/lib.rs*

~~~rust
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    client::connect();
  }
}
~~~

~~~
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `client`
~~~

* `tests` 모듈은 예외적으로 이 스코프 내에서 `client` 모듈이 필요함
* 해결방법: `tests` 모듈 내에 `super` 선언하여 가져오기

~~~rust
#[cfg(test)]
mod tests {
  use super::client;

  #[test]
  fn it_works() {
    client::connect();
  }
}
~~~
