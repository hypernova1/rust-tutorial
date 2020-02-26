
# Rust Tutorial
  
## 목적  
- Rust 언어 학습  
  
## 계획  
- https://rinthel.github.io/rust-lang-book-ko 사이트 학습    
  
<br>  
<br>  
<br>    
  
## 목차  
[1. 시작하기](#1-시작하기)    
* [1.1. 설치하기](#11-설치하기)    
* [1.2. Hello, World!](#12-hello-world)  
* [1.3. Hello, Cargo!](#13-hello-cargo)  

[2. 추리게임 튜토리얼](#2-추리게임-튜토리얼)  
  
[3. 보편적인 프로그래밍 개념](#3-보편적인-프로그래밍-개념)
* [3.1. 변수와 가변성](#31-변수와-가변성)
* [3.2. 데이터 타입들](#32-데이터-타입들)
* [3.3. 함수 동작 원리](#33-함수-동작-원리)
* [3.4. 주석](#34-주석)
* [3.5. 제어문](#35-제어문)  

[4. 소유권(Ownership)](#4-소유권ownership)
* [4.1 소유권이란?](#41-소유권이란)
* [4.2 참조자(References)와 빌림(Borrowing)](#42-참조자references와-빌림borrowing)
* [4.3 슬라이스(Slices)](#43-슬라이스slices)
# 1. 시작하기
## 1.1. 설치하기
### Linux와 MacOS에서 Rustup 설치 커맨드 (러스트 안정화 버전)
~~~
$ curl https://sh.rustup.rs -sSf | sh
~~~

### 1. 커맨드 재시작 없이 바로 시작하기
~~~
$ source $HOME/.cargo/env
~~~

### 2. ~/bash_profile에 추가하기
~~~
$ export PATH="$HOME/.cargo/bin:$PATH"
~~~

### Windows에서 Rustup 설치
<https://www.rust-lang.org/en-US/install.html> 접속 후 설치 진행

### 버전확인
~~~
rustc --version
~~~
### 로컬 문서 실행
~~~
rustup doc
~~~
  
<br>

## 1.2. Hello, World!
### 프로그램 작성 및 실행
Filename: main.rs
~~~rust
fn main() {
   println!("Hello world!");
}
~~~
~~~
$ rustc main.rs
$ ./main        <- Linux or MacOS
> .\main.exe    <- Windows
Hello, world!
~~~
* 러스트는 탭이 아닌 네개의 스페이스로 들여쓰기를 함
* `println!`: 러스트 매크로(함수가 아님)
* 컴파일과 실행은 개별적 단계
<br>

## 1.3. Hello, Cargo!
### **Cargo**(카고)
* 러스트 빌드 시스템 및 패키지 매니저 (rustup 설치시 자동으로 설치)
### 1. 버전확인
~~~
$ cargo --version
~~~

### 2. 프로젝트 생성
~~~
$ cargo new [project name] --bin
~~~
* 옵션: `--lib`(라이브러리), `--bin`(실행파일) ...

### 3. 프로젝트 구조
~~~
project/
    -src/       <-- 소스파일 디렉토리
    -Cargo.lock
    -Cargo.toml <-- Cargo 환경설정 파일
~~~

### 4. Cargo.toml 구조
~~~
[package]                                   <-- 프로젝트 정보
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
[dependencies]                              <-- 의존성 리스트 섹션
~~~

### 5. 프로젝트 빌드 및 실행
* 빌드
    * 파일 생성 위치
        * `./target/debug/[project name]` (Linux, MacOS)
        * `.\target\debug\[project name].exe` (Windows)
~~~
$ cargo build
~~~
* 실행 (빌드 + 실행)
~~~
$ cargo run
~~~
* 컴파일 확인 (실행파일 생성 X)
~~~
$ cargo check
~~~
* 릴리즈 빌드
    * 최적화와 함께 컴파일
~~~
$ cargo build --release
~~~


### 6. 관련 문서
<https://doc.rust-lang.org/cargo/>

<br>

# 2. 추리게임 튜토리얼
<br>


# 3. 보편적인 프로그래밍 개념

## 3.1. 변수와 가변성
### 1. 기본 변수(let, const)는 불변성 (*immutable*)
~~~
let x = 0;
x = 1 //컴파일 에러
~~~

### 2. mut 접두어를 붙이면 가변성 (*mutable*)
~~~rust
let mut x = 0;
x = 1; //OK
~~~

### 3. 상수(*const*)
* `mut` 키워드 사용 불가
* 어느 영역에서든지 선언 가능
* 선언되어 있는 영역 내에서 프로그램이 실행되는 동안 항상 유효함
* 상수 표현식만 사용 가능 (함수 호출 결과값 및 실행시간에 결정되는 값이 할당될 수 없음)
~~~
const MAX_POINTS: u32 = 100_000;
~~~

### 4. Shadowing
~~~
let x = 5;
let x = x + 1;
let x = x * 2;
println!("{}", x); // 12
~~~
* `mut` 키워드와의 차이점
    * 변경 후에는 불변성 유지
    * 타입 변경 가능 (불필요한 변수 선언 방지)
~~~
let spaces = "   ";
let spaces = spaces.len(); // OK

let mut spaces = "   ";
spaces = spaces.len(); // 컴파일 에러
~~~
<br>

## 3.2. 데이터 타입들
### 1. 데이터 타입 명시
~~~
let guess: u32 = "32".parse().expect("Not a number!");
~~~
### **2. 스칼라 타입들**
* 스칼라: 하나의 값으로 표현되는 타입
    * 종류: 정수형, 부동소수점 숫자, boolean, 문자

#### 2.1.1. 정수형: 소수점이 없는 숫자
| Length | Signed | Unsigned |
|:--------|:--------|--------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| arch | isize | usize |
* 각부호의 변수는 -(2<sup>n-1</sup>) ~ 2<sup>n-1</sup> - 1 까지의 값을 표괄 (n: 사용되는 타입의 비트수)
    * ex) `u8`: 0 ~ 2<sup>8</sup>-1 (= 0~255)
* isize, usize 타입은 운영체제의 bit 수에 따라 결정됨
* 시각적 구분을 위해 `_`를 사용
    * ex) `1_000`
    * byte 리터럴을 제외한 모든 정수형은 `57u8`과 같은 타입의 접미사와 `_` 사용 가능
    
#### 2.1.2.  정수형 리터럴들
| Number literals | Example |
|:--------|:--------|
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8` only) | `b'A'` | 

#### 2.2. 부동 소수점 타입
* 소수점을 갖는 타입
* IEEE-754 표준에 따라 표현됨
* `f32` (1배수 정밀도), `f64` (2배수 정밀도)
~~~rust
let x = 2.0;        // f64
let y: f32 = 3.0    // f32
~~~

#### 2.3. Boolean 타입
* `true` 와 `false` 로 표현
~~~rust
let t = true;
let f: bool = false;
~~~ 

#### 2.4. 문자타입
* char 타입
~~~rust
let c = 'z';
let z = 'Z';
~~~

### 3. 복합 타입들
#### 3.1 튜플
* 다양한 타입의 숫자들의 집합
~~~rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 구조해체
let (x, y, z) = tup;

// 값 할당 (인덱스 사용)
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
~~~

#### 3.2 배열
* 여러 값들의 집합체
* 튜플과 다르게 모든 요소는 **같은 타입**이어야 함
* 한 번 선언되면 크기를 변경할 수 없음 (변경하고 싶으면 *vector* 를 사용)
* heap 보다 stack 에 할당하는 것을 원할때 사용 (고정된 숫자의 요소를 갖는다 확신할 때)
~~~rust
let a = [1, 2, 3, 4, 5];

let first = a[0]; //첫번째 요소 접근
~~~
* 배열의 길이를 초과하는 요소에 접근하면 에러 발생 (rust에서는 panic 하다라고 표현하는 듯)
<br>

# 3.3 함수 동작 원리
### 1. 함수 선언하기
~~~rust
fn another_function() {
    println!("Another function");
}
~~~
### 2. 함수의 매개 변수
~~~rust
fn main() {
    another_function(5);
}

fn another_function(x: u32) { //타입 정의
    println!("{}", x);
}
~~~

### 3. 함수 본문
#### 3.1 구문과 표현식
* 구문
~~~rust
let y = 6; //구문이기 때문에 반환값이 없음

let x = (let y = 6); //error
~~~
* 함수
~~~rust
fn main() {
    let x = 5;
    
    let y = { //표현식부
        let x = 3;
        x + 1 //반환값은 뒤에 세미콜론을 붙이지 않음
    };
    println!("{}", y); //4
}
~~~
### 4. 반환 값을 갖는 함수
~~~rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    
    println!("{}", x); //5
}
~~~
#### 4.1. 잘못된 함수 반환 값의 예
~~~rust
fn main() {
    let x = plus_one(5);

    println!("{}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;  //반환 값이 아니라 구문이기 때문에 error
}           //비어있는 튜플로 반환 됨 -> '()'
~~~


## 3.4. 주석

## 3.5. 제어문
### 1. `if` 표현식
~~~rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
~~~
* 코드의 조건은 반드시 `bool`이어야 함
~~~rust
let number = 3;

if number {
    println!("number is three"); //bool이 아니고 정수형이라서 error
}
~~~

### 2. `else if`
~~~rust
let number = 6;

if number % 2 == 0 {
    println!("number is even");
} else if number % 2 == 1 {
    println!("number is odd");
} else {
    println!("number is error");
}
~~~

### 3. `let`구문에서 `if`사용하기
~~~rust
let condition = ture;
let number  = if condition {
    5
} else {
    6
};
~~~

### 4. 반복문과 반복
#### 4.1 `loop`
~~~rust
loop {
    println!("againt");
}
~~~
* 프로그램을 강제 종료하기 전까지 again 반복
#### 5.2 `while`
~~~rust
let mut number = 3;

while number != 0 {
    println!("{}", number);

    number = number - 1;
}
~~~
### 5.2 `for`을 사용하여 콜렉션 반복
~~~rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("{}", element);
}
~~~
~~~rust
for number in (1..4).rev() { //(1..4) => Range: 한 숫자에서 다른 숫자까지 모든 숫자를 차례로 생성
    println!("{}", number);
}
~~~
<br>
<br>

## 4. 소유권(Ownership)
### 4.1 소유권이란
#### 1. 스택과 힙
스택과 힙 모두 코드상에서 런타임시에 사용할 수 있는 메모리지만 각기 다른 방식으로 구조화되어 있음
* 스택
    * 받아들인 순서대로 값을 저장하고 반대 방향으로 값을 지움(LIFO)
    * 데이터에 접근하는 방식 덕분에 속도가 빠름 (데이터를 가져올 곳이 항상 top 이기 때문)
    * 스택에 담긴 모든 데이터는 고정된 크기를 가져야 함
* 힙
    * 컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터
    * 스택보다 느림
    * 저장 방식
        1. 저장공간이 있는지 확인
        2. 운영체제에게 공간을 할당 받아 포인터를 돌려 받음
        * 스택에 포인터를 저장하는 것은 할당이라 표현하지 않음  
* 스택에 포인터를 저장할 수 있지만 실제 데이터 사용시에는 포인터를 따라가야 함

#### 2. 소유권 규칙
1. 각각의 값은 해당값의 오너(*owner*)를 갖고 있음
2. 한 번에 딱 하나의 오너만 존재
3. 오너가 스코프 밖으로 벗어나면, 값은 버려짐(*dropped*)

#### 3. 변수의 스코프
~~~rust
{                       //s가 유효하지 않음 (선언되기 전)
    let s = "hello";    //s는 이 지점부터 유효
}                       //스코프가 끝났기 때문에 s는 유효하지 않음
~~~

#### 4. `String`타입
* 일반적인 문자열은 불변
* 만일 고정적이지 않은 값을 입력받고 싶다면 `String`을 써야함
##### 4.1 선언 방법
~~~rust
let s = String::from("hello"); //'::': 네임스페이스 연산자
~~~
#### 4.2 문자열 변경
~~~rust
let mut s = String::from("Hello");

s.push_str(", world!"); //push_str()은 해당 스트링 리터럴을 스트링에 붙임
println!("{}", s); //Hello, world!
~~~

#### 5. 메모리와 할당
* 스트링 리터럴은 값을 컴파일시에 알 수 있기 때문에 빠르고 효율적이지만 문자열을 변경할 수가 없음
* `String`타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어짐
    * 런타임시에 운영체제로부터 메모리가 요청되어야 함
        * ex) `String::from`
    * `String`의 사용이 끝나면 운영체제에게 메모리를 반납해야함
        * 스코프를 벗어나면 반납

#### 6. rust의 반납 방식
~~~rust
{
    let s = String::from("hello"); //s는 여기서부터 유효함
    //s 사용
}                                  //스코프가 끝나고 s는 유효하지 않음(drop 호출)
~~~

#### 7. 이동(`move`)
~~~rust
let s1 = String::from("hello");
let s2 = x; //s1의 값이 s2로 이동 됨
println!("{}", x); //error
~~~
* 이동 순서
    1. `s1`을 `s2`에 대입시 String의 데이터가 복사됨 (포인터, 길이 값, 용량)
    ![1](https://rinthel.github.io/rust-lang-book-ko/img/trpl04-02.svg)
        * `s1`, `s2`가 모두 하나의 포인터를 참조(얕은 복사)
    2. `s1` 무효화(메모리 해제)
    ![2](https://rinthel.github.io/rust-lang-book-ko/img/trpl04-04.svg)


#### 8. 클론(`clone`)
~~~rust
let s1 = String::from("hello");
let s2 = s1.clone(); //데이터 복제(깊은 복사)

println!("{}, {}", s1, s2); //OK
~~~

#### 9. 복사(`copy`)
* 정수형과 같은 컴파일시 크기가 결정되는 타입들은 스택에 저장되기 때문에 실제 복사본이 빠르게 만들어질 수 있음
~~~rust
let x = 5;
let y = x;

println!("{}-{}", x, y); //OK
~~~
    
#### 10. 소유권과 함수
~~~rust
fn main() {
    let s = String::from("hello");  //s가 스코프안으로 들어옴
    
    takes_ownership(s);              //s의 값이 함수안으로 이동
    // println!("{}", s)             //error! s는 더이상 유효하지 않음
    let x = 5;                      //x가 스코프안으로 들어옴
    
    makes_copy(x);                  //i32타입은 copy가 되므로 x를 계속 사용 가능

} //s는 이미 이동되었으므로 아무일도 발생하지 않음

fn takes_ownership(some_string: String) { //some_string이 스코프 안으로 들어옴
    println!("{}", some_string); //hello
} //some_string이 스코프 밖으로 벗어나 drop 호출 (메모리 해제)

fn makes_copy(some_integer: u32) { //some_integer가 스코프 안으로 들어옴
    println!("{}", some_integer); //5
} //아무일도 발생하지 않음
~~~

#### 11. 반환 값과 스코프
~~~rust
fn main() {
    let s1 = gives_ownership();         //반환 값을 s1에게 이동 시킴

    let s2 = String::from("hello")      //s2가 스코프 안으로 들어옴

    let s3 = takes_and_gives_back(s2);  //s2가 함수 안으로 이동되고 반환값이 s3으로 이동
} //s1, s3은 스코프 밖으로 벗어나서 drop 호출 s2는 이동되어서 아무일도 일어나지 않음

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
~~~

* 튜플을 이용하여 여러 값 돌려받기
~~~rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
~~~
<br>

### 4.2. 참조자(References)와 빌림(Borrowing)
#### 1. 함수에 값을 넘길 때 *소유권*(Ownership)을 넘기는 대신 개체에 대한 *참조자*(References)를 인자로 사용
~~~rust
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len); //OK

}

fn calculate_length(s: &String) -> usize { //s는 스트링의 참조자형(빌림)
  s.len()
} //s는 스코프를 벗어났지만 가리키고 있는 값에 대한 소유권이 없기 때문에 아무일도 발생하지 않음
~~~

#### 2. 참조자 변경은 기본적으로 허용되지 않음(*불변*)
~~~rust
fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String) {
  some_string.push_str(", world"); //error
}
~~~

#### 3. 가변 참조자(Mutalble References)
~~~rust
fn main() {
  let mut s = String::from("hello");

  change(&mut s);
}

fn change(some_string: &mut String) { //some_string은 가변 참조자
  some_string.push_str(", world"); //OK
}
~~~
  * 특정한 스코프 내에서 특정한 데이터 조각에 대한 가변참조자를 딱 하나 생성 가능
~~~rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; //error
~~~
* 데이터 레이스를 방지하기 위함
    * 데이터 레이스는 아래의 세가지 동작이 발생했을 때 나타나는 레이스 조건
        1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근
        2. 그 중 적어도 하나의 포인터가 데이터를 씀
        3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없음

##### 3.1 데이터 레이스 방지
~~~rust
let mut s = String::from("hello");
{
  let r1 = &mut s;
} //r1은 스코프를 벗어났으므로 drop

let rs = &mut s; //OK
~~~

##### 3.2 가변 참조자와 불변 참조자 혼용
~~~rust
let mut s = String::from("hello");

let r1 = &s; //OK
let r2 = &s; //OK
let r2 = &mut s; //error
~~~
* 불변 참조자를 가지고 있을 동안에도 가변 참조자를 만들 수 없음

#### 4. 댕글링 참조자(Dangling References)
~~~rust
fn main() {
  let reference_to_nothing = dangle(); //error
}

fn dangle() -> &String { // String 참조자를 반환
  let s = String::from("hello"); //s는 새로운 String

  &s // s의 참조자를 반환
} //s가 스코프를 벗어나서 drop 됨(메모리가 사라짐)
~~~
* 해결 방법: `String`을 직접 반환
~~~rust
fn no_dangle() -> String {
  let s = String::from("hello");

  s
}
~~~
### **참조자의 규칙**
1. 어떤 경우든 아래 항목의 하나만 가질 수 있음
   * 하나의 가변 참조자
   * 임의 개수의 불변 참조자
2. 참조자는 항상 유효해야함


## 4.3. 슬라이스(Slices)
### 1. 소유권을 갖지 않는 타입
~~~rust
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}
~~~

#### 코드 상세
~~~rust
let bytes = s.as_bytes();
~~~
1.  `String`을 `byte`배열로 변환

~~~rust
for (i, &item) in bytes.iter().enumerate() {}
~~~
2.  `iter`: `byte`배열의 각 요소 반환
3.  `enumerate`: `iter`의 결과 값 직접 반환 대신 감싸서 *튜플*로 반환
      * `for`루프 내에서 `i`는 인덱스, `&item`은 튜플 내의 한 바이트에 대응
      * `iter().enumerate()`의 요소에 대한 참조자를 가지므로 `&` 사용

~~~rust
  if item == b' ' {
    return i;
  }
}

s.len()
~~~
4. 공백 문자를 찾았다면 위치(index)를 반환  
5. 아니면 `String`의 길이를 반환
* 문제점
  * `usize`를 반환하고 있지만 `&String`의 내용물 안에서만 유효
  * `len`이 기존의 `String`으로 부터 분리되어 있기 때문에 나중에도 여전히 유효한지 알 수 없음

### 2. `String`과 `len`분리 시키지 않는 로직
#### 1. 기존 로직의 문제점
~~~rust
fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s);  //5

  s.clear();                  //String을 비워서 ""로 만듬

  //word는 여전히 5를 가지고 있지만 5라는 값을 의미있게 쓸 수 있는 String은 없음
} //word drop
~~~

### 2. 해결방법 : 스트링 슬라이스
#### 2.1. 스트링 슬라이스(`&str`)
~~~rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
~~~
![슬라이스](https://rinthel.github.io/rust-lang-book-ko/img/trpl04-06.svg)

#### 2.2. 스트링 슬라이스 반환
~~~rust

fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s);

  // s.clear(); //error
  println!("{}", word);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
~~~
* `clear`함수가 `String`을 잘라낼 때 가변 참조자를 가지지 못해서 생기는 오류 (이미 가변 참조자가 사용되고 있기 때문)

#### 2.3. 스트링 리터럴은 슬라이스이다.
#### 2.4. 파라미터로서의 스트링 슬라이스
~~~rust
fn main() {
  let my_string = String::from("hello world");

  //first_word가 'String'의 슬라이스로 동작함
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  //my_string_literal이 'String'슬라이스로 동작함
  let word = first_word(&my_string_leteral[..]);

  //스트링 리터럴은 슬라이스이기 때문에 동작
  let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str { //&String -> &str
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
~~~

#### 2.5. 그 밖의 슬라이스들
~~~rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
~~~
