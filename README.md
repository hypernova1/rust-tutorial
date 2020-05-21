
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
* [4.1. 소유권이란?](#41-소유권이란)
* [4.2. 참조자(References)와 빌림(Borrowing)](#42-참조자references와-빌림borrowing)
* [4.3. 슬라이스(Slices)](#43-슬라이스slices)  

[5. 구조체](#5-구조체)
* [5.1. 구조체를 정의하고 생성하기](#51-구조체를-정의하고-생성하기)
* [5.2. 구조체를 이용한 예제 프로그램](#52-구조체를-이용한-예제-프로그램)
* [5.3. 메소드 문법](#53-메소드-문법)

[6. 열거형과 패턴 매칭](#6-열거형과-패턴-매칭)
* [6.1. 열거형 정의하기](#61-열거형-정의하기)
* [6.2. match 흐름 제어 연산자](#62-match-흐름-제어-연산자)
* [6.3. it let을 사용한 간결한 흐름 제어](#63-if-let을-사용한-간결한-흐름-제어)

[7. 모듈](#7-모듈)
* [7.1. mod와 파일 시스템](#71-mod와-파일-시스템)
* [7.2. pub으로 가시성 제어하기](#72-pub으로-가시성-제어하기)
* [7.3. use로 이름 가져오기](#73-use로-이름-가져오기)

[8. 컬렉션(Collections)](#8-컬렉션Collections)
* [8.1. 벡터](#81-벡터)
* [8.2. 스트링](#82-스트링)
* [8.3. 해쉬맵](#83-해쉬맵)


<br>

<br>

<br>


# 1. 시작하기

## 1.1. 설치하기
### Linux와 MacOS에서 Rustup 설치 커맨드 (러스트 안정화 버전)
~~~
$ curl https://sh.rustup.rs -sSf | sh
~~~

### I. 커맨드 재시작 없이 바로 시작하기
~~~
$ source $HOME/.cargo/env
~~~

### II. ~/bash_profile에 추가하기
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
### I. 버전확인
~~~
$ cargo --version
~~~

### II. 프로젝트 생성
~~~
$ cargo new [project name] --bin
~~~
* 옵션: `--lib`(라이브러리), `--bin`(실행파일) ...

### III. 프로젝트 구조
~~~
project/
    -src/       <-- 소스파일 디렉토리
    -Cargo.lock
    -Cargo.toml <-- Cargo 환경설정 파일
~~~

### IV. Cargo.toml 구조
~~~
[package]                                   <-- 프로젝트 정보
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
[dependencies]                              <-- 의존성 리스트 섹션
~~~

### V. 프로젝트 빌드 및 실행
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


### VI. 관련 문서
<https://doc.rust-lang.org/cargo/>

<br>

# 2. 추리게임 튜토리얼
<br>


# 3. 보편적인 프로그래밍 개념

## 3.1. 변수와 가변성
### I. 기본 변수(let, const)는 불변성 (*immutable*)
~~~
let x = 0;
x = 1 //컴파일 에러
~~~

### II. mut 접두어를 붙이면 가변성 (*mutable*)
~~~rust
let mut x = 0;
x = 1; //OK
~~~

### III. 상수(*const*)
* `mut` 키워드 사용 불가
* 어느 영역에서든지 선언 가능
* 선언되어 있는 영역 내에서 프로그램이 실행되는 동안 항상 유효함
* 상수 표현식만 사용 가능 (함수 호출 결과값 및 실행시간에 결정되는 값이 할당될 수 없음)
~~~
const MAX_POINTS: u32 = 100_000;
~~~

### IV. Shadowing
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
### I. 데이터 타입 명시
~~~
let guess: u32 = "32".parse().expect("Not a number!");
~~~
### **II. 스칼라 타입들**
* 스칼라: 하나의 값으로 표현되는 타입
    * 종류: 정수형, 부동소수점 숫자, boolean, 문자

#### II.i. 정수형: 소수점이 없는 숫자
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
    
#### II.ii.  정수형 리터럴들
| Number literals | Example |
|:--------|:--------|
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8` only) | `b'A'` |

#### II.iii. 부동 소수점 타입
* 소수점을 갖는 타입
* IEEE-754 표준에 따라 표현됨
* `f32` (1배수 정밀도), `f64` (2배수 정밀도)
~~~rust
let x = 2.0;        // f64
let y: f32 = 3.0    // f32
~~~

#### II.iv. Boolean 타입
* `true` 와 `false` 로 표현
~~~rust
let t = true;
let f: bool = false;
~~~

#### II.v. 문자타입
* char 타입
~~~rust
let c = 'z';
let z = 'Z';
~~~

### III. 복합 타입들
#### III.i 튜플
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

#### III.ii 배열
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
### I. 함수 선언하기
~~~rust
fn another_function() {
    println!("Another function");
}
~~~
### II. 함수의 매개 변수
~~~rust
fn main() {
    another_function(5);
}

fn another_function(x: u32) { //타입 정의
    println!("{}", x);
}
~~~

### III. 함수 본문
#### III.i. 구문과 표현식
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
### IV. 반환 값을 갖는 함수
~~~rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    
    println!("{}", x); //5
}
~~~
#### IV.i. 잘못된 함수 반환 값의 예
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
### I. `if` 표현식
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

### II. `else if`
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

### III. `let`구문에서 `if`사용하기
~~~rust
let condition = ture;
let number  = if condition {
    5
} else {
    6
};
~~~

### IV. 반복문과 반복
#### IV.i `loop`
~~~rust
loop {
    println!("againt");
}
~~~
* 프로그램을 강제 종료하기 전까지 again 반복
#### IV.ii `while`
~~~rust
let mut number = 3;

while number != 0 {
    println!("{}", number);

    number = number - 1;
}
~~~
### IV.iii `for`을 사용하여 콜렉션 반복
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
#### I. 스택과 힙
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

#### II. 소유권 규칙
1. 각각의 값은 해당값의 오너(*owner*)를 갖고 있음
2. 한 번에 딱 하나의 오너만 존재
3. 오너가 스코프 밖으로 벗어나면, 값은 버려짐(*dropped*)

#### III. 변수의 스코프
~~~rust
{                       //s가 유효하지 않음 (선언되기 전)
    let s = "hello";    //s는 이 지점부터 유효
}                       //스코프가 끝났기 때문에 s는 유효하지 않음
~~~

#### IV. `String`타입
* 일반적인 문자열은 불변
* 만일 고정적이지 않은 값을 입력받고 싶다면 `String`을 써야함
##### IV.i 선언 방법
~~~rust
let s = String::from("hello"); //'::': 네임스페이스 연산자
~~~
#### IV.ii 문자열 변경
~~~rust
let mut s = String::from("Hello");

s.push_str(", world!"); //push_str()은 해당 스트링 리터럴을 스트링에 붙임
println!("{}", s); //Hello, world!
~~~

#### V. 메모리와 할당
* 스트링 리터럴은 값을 컴파일시에 알 수 있기 때문에 빠르고 효율적이지만 문자열을 변경할 수가 없음
* `String`타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어짐
    * 런타임시에 운영체제로부터 메모리가 요청되어야 함
        * ex) `String::from`
    * `String`의 사용이 끝나면 운영체제에게 메모리를 반납해야함
        * 스코프를 벗어나면 반납

#### VI. rust의 반납 방식
~~~rust
{
    let s = String::from("hello"); //s는 여기서부터 유효함
    //s 사용
}                                  //스코프가 끝나고 s는 유효하지 않음(drop 호출)
~~~

#### VII. 이동(`move`)
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


#### VIII. 클론(`clone`)
~~~rust
let s1 = String::from("hello");
let s2 = s1.clone(); //데이터 복제(깊은 복사)

println!("{}, {}", s1, s2); //OK
~~~

#### IX. 복사(`copy`)
* 정수형과 같은 컴파일시 크기가 결정되는 타입들은 스택에 저장되기 때문에 실제 복사본이 빠르게 만들어질 수 있음
~~~rust
let x = 5;
let y = x;

println!("{}-{}", x, y); //OK
~~~

#### X. 소유권과 함수
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

#### XI. 반환 값과 스코프
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
#### I. 함수에 값을 넘길 때 *소유권*(Ownership)을 넘기는 대신 개체에 대한 *참조자*(References)를 인자로 사용
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

#### II. 참조자 변경은 기본적으로 허용되지 않음(*불변*)
~~~rust
fn main() {
  let s = String::from("hello");

  change(&s);
}

fn change(some_string: &String) {
  some_string.push_str(", world"); //error
}
~~~

#### III. 가변 참조자(Mutalble References)
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

##### III.i 데이터 레이스 방지
~~~rust
let mut s = String::from("hello");
{
  let r1 = &mut s;
} //r1은 스코프를 벗어났으므로 drop

let rs = &mut s; //OK
~~~

##### III.ii 가변 참조자와 불변 참조자 혼용
~~~rust
let mut s = String::from("hello");

let r1 = &s; //OK
let r2 = &s; //OK
let r2 = &mut s; //error
~~~
* 불변 참조자를 가지고 있을 동안에도 가변 참조자를 만들 수 없음

#### IV. 댕글링 참조자(Dangling References)
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
### I. 소유권을 갖지 않는 타입
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

### II. `String`과 `len`분리 시키지 않는 로직
#### II.i. 기존 로직의 문제점
~~~rust
fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s);  //5

  s.clear();                  //String을 비워서 ""로 만듬

  //word는 여전히 5를 가지고 있지만 5라는 값을 의미있게 쓸 수 있는 String은 없음
} //word drop
~~~

#### II. 해결방법 : 스트링 슬라이스
#### II.i. 스트링 슬라이스(`&str`)
~~~rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
~~~
![슬라이스](https://rinthel.github.io/rust-lang-book-ko/img/trpl04-06.svg)

#### II.ii. 스트링 슬라이스 반환
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

#### II.iii. 스트링 리터럴은 슬라이스이다.
#### II.iv. 파라미터로서의 스트링 슬라이스
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

#### II.v. 그 밖의 슬라이스들
~~~rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
~~~

<br>

## 5. 구조체
### 5.1. 구조체를 정의하고 생성하기
#### I. 구조체 정의
~~~rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
~~~
* 구조체의 구성요소들은 각각 다른 타입을 가질 수 있음

#### II. 구조체의 인스턴스 생성
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

#### III. 구조체 갱신법(`..`)
~~~rust
let user2 = User {
  email: String::from("another@gmail.com"),
  username: String::from("another"),
  ..user1
}
~~~

#### IV. 튜플 구조제
* 이름이 없고 필드마다 타입이 다름
~~~rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
~~~

> #### 구조체 데이터의 소유권
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

### 5.2. 구조체를 이용한 예제 프로그램

### 5.3. 메소드 문법
* 메소드는 함수와 유사
* `fn` 키워드로 선언
* 함수와 달리 구조체의 내용 안에 정의
* 첫번째 파라미터는 언제나 `self`
  * `self`: 메소드가 호줄되고 있는 구조체의 인스턴스를 가리킴

#### I. 메소드 정의
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

#### II. 더 많은 파라미터를 가진 메소드
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

#### III. 연관 함수
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

<br>

## 6. 열거형과 패턴 매칭
### 6.1. 열거형 정의하기
#### I. 정의하기
~~~rust
enum IpAddrKind {
  V4,
  V6,
}
~~~
#### II. 열거형 값
~~~rust
let four = IpAddrKind::A4
let six = IpAddrKind::A6
~~~
#### III. 사용하기
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

#### III. 여러 데이터 타입 정의하기
~~~rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
~~~

#### IV. 메소드 정의하기
~~~rust
impl Message {
  fn call(&self) { //self => "hello"
    //...
  }
}

let m = Message::Write(String::from("hello"));
m.call();
~~~

#### V. `Option`
##### V.i. 구조
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

### 6.2 `match` 흐름 제어 연산자
#### I. 예제
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
#### II. 예제2
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

#### III. 값을 바인딩하는 패턴들
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

#### IV. `Option<T>`를 이용하는 매칭
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

#### V. `match` 규칙
~~~rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
  }
}
~~~
* `match`는 하나도 빠트리면 안됨
* 위의 예제의 경우 `None`을 다루지 않았기 때문에 컴파일 에러

#### VI. `_`변경자(placeholder)
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

### 6.3 `if let`을 사용한 간결한 흐름 제어
#### I. `if let`
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

#### II. `if let`과 `else`
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

<br>

## 7. 모듈
### 7.1. `mod`와 파일 시스템
#### I. 라이브러리 크레이트 만들기
~~~
$ cargo new communicator --lib
$ cd communicator
~~~

#### II. 내부 코드
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


#### III. 모듈 정의
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

#### IV. 모듈 내부에 모듈 정의
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

##### V. 모듈을 다른 파일로 옮기가
* IV의 에제는 시스템이 복잡해질수록 유지보수하기가 힘들기 때문에 따로 폴더 구조를 만듬  

##### 폴더 구조
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

#### VI. `network` 모듈 개별 파일로 추출하기

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

#### VII. `server` 모듈 추출하기
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

#### VIII. 에러 해결
1. network 디렉토리를 생성해서 그 안에 *src/network* 파일 옮기기
2. *src/network/mod.rs* 생성
##### 폴더구조
~~~
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
~~~

#### 모듈 파일 시스템의 규칙
* `foo`라는 이름의 서브 모듈을 가지고 있다면 *foo.rs*라는 이름의 파일 내에 `foo`에 대한 선언이 있어야함
* `foo`가 서브모듈을 가지고 있다면 *foo/mod.rs*라는 이름의 파일에 `foo`에 대한 선언이 있어야함
~~~
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
~~~

### 7.2. `pub`으로 가시성 제어하기
#### I. 바이너리 크레이트 만들기

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

#### 빌드시 나타나는 오류
~~~
$ cargo build

error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 
~~~
* `client` 모듈이 비공개라서 접근이 불가능


#### II. `pub` 키워드를 사용해서 함수를 공개로 만들기
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

#### `cargo build`시 생기는 경고
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

#### `network` 모듈 공개
Filename: *src/lib.rs*
~~~rust
pub mod client;
pub mod network;
~~~


#### 비공개 규칙
1. 어떤 아이템이 공개라면, 부모의 모듈 어디서든지 접근 가능
2. 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능

##### 예제
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

### 7.3. use로 이름 가져오기
#### use를 사용하지 않았을 때
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

## 8. 컬렉션(Collections)
### 8.1. 벡터(`Vec<T>`)
* 메모리 상에서 서로 이웃하도록 모든 값을 집어 넣는 단일 데이터 구조 안에서 하나 이상의 값을 저장
* 같은 타입만 저장 가능

### I. 생성하기
#### I.i. `Vec::new` 함수 호출
~~~rust
let v: Vec<i32> = Vec::new();
~~~
#### I.ii. `vec!`매크로 사용
~~~rust
let v = vec![1, 2, 3];
~~~

### II. 갱신하기
~~~rust
let mut v = Vec::new();
v.puch(5);
v.puch(6);
v.puch(7);
v.puch(8);
~~~

### III. 드롭
* 모든 요소가 드롭이 됨
~~~rust
{
  let v = vec![1, 2, 3, 4];
} //v가 스코프 밖으로 벗어났으므로 모든 데이터 해제
~~~

### IV. 요소 읽기
~~~rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
~~~

#### 길이를 벗어난 데이터에 접근하려고 할 때
~~~rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
~~~
* 첫번째 `deos_not_exist`는 `panic!`을 일으킴
  * 존재하지 않는 요소를 참조하기 때문
* 첫번째 `deos_not_exist`는 패닉 없이 `None`이 반환 됨


#### 유효하지 않은 참조자
~~~rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
~~~
* 아이템에 대한 참조자를 가지는 동안 벡터에 요소 추가 시도
* 아래와 같은 에러 발생

~~~
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as
immutable
  |
4 | let first = &v[0];
  |              - immutable borrow occurs here
5 |
6 | v.push(6);
  | ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
~~~
* 새로운 요소를 끝에 추가하면 새로 메모리를 할당 받기 때문에 예전 요소들을 새 공간에 복사하는 일이 필요할 수도 있는데 이는 벡터가 도든 요소들을 붙여서 저장할 공간이 충분치 않은 환경에서 일어날 수 있음 이러한 경우에 첫번째 요소할당(`let first = &v[0]`)이 해제된 메모리를 가르킬 수 있음
* 빌림 규칙은 이러한 상황에 빠지지 않도록 함


### V. 반복처리
#### V.i. `for`루프를 이용한 반복
~~~rust
let v = vec![100, 32, 57];
for i in & v {
  println!("{}", i);
}
~~~

#### V.ii. 가변 참조자를 사용하여 요소 변경하기
~~~rust
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
~~~
* `*`(역참조 연산자)를 이용하여 값을 얻어야함

### 열거형을 사용하여 여러타입 저장
~~~rust
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String).
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Float(2.3),
  SpreadsheetCell::String(String::from("blue")),
];
~~~

### 8.2. 스트링
* 스트링 슬라이스(`str`)와 다르게 `String`타입은 표준 라이브러리를 통해 제공됨
* UTF-8 인코딩

#### I. 생성하기
~~~rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string(); //String::from("initial contents")와 같음

let s = "initial contents".to_string();
~~~

#### II. 갱신하기
##### II.i `push_str`, `push`
~~~rust
let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(&s2);

println!("{}", s2); //bar


let mut s = String::from("lo");
s.push('l');
~~~

##### II.ii `+`연산자나 `format!`매크로를 이용한 접합
~~~rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; //여기서 s1은 이동되어 더 이상 사용 불가
~~~

* `add` 메소드 시그니처

~~~rust
fn add(self, s: &str) -> String { // self의 소유권을 가져가기 때문에 메소드 종료시 drop 됨
~~~

~~~rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "=" + &s2 + "-" + &s3 //일반 스트링 연산
let s = format!("{}-{}-{}", s1, s2, s3); //매크로를 사용한 연산: 소유권을 가져가지 않음
~~~

##### II.iii 스트링 내부의 인덱싱

* 러스트는 `String` 의 인덱싱을 지원하지 않는다.

~~~rust
let s1 = String::from("hello");
let h = s1[0]; //error
~~~

* 내부적 표현
  * `String`은 `Vec<u8>`을 감싼 것(wrapper)

~~~rust
let len = String::from("Hela").len();
~~~

* `Vec`이 4바이트 길이라 `len`의 값은 4

~~~rust
let len = String::from("Здравствуйте").len();
let len = "Здравствуйте"; 

let answer = &hello[0];
~~~

* UTF-8로 인코딩되었기 때문에 `len`은 24바이트
* `3` 의 첫 번째 바이트는 208이고 두 번째 바이트트 151이라 208이 반환될 거 같지만 그 자체로 유효한 문자가 아니기 때문에 러스트는 컴파일 패닉를 발생시킴



### III. 스트링 슬라이싱

~~~rust
let hello = "Здравствуйте";

let s = &hello[0..4];
~~~

* `s` 는 4바이트를 가진 `&str`이고 값은 `Зд`
* 만약 `&hello[0..1]`을 호출한다면 런타임 패닉을 발생시킴

### IV. 스트링 내에서 반복적으로 실행되는 메소드

1. `char`

~~~rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
~~~

* 결과값

~~~rust
न
म
स
्
त
े
~~~

2. `bytes`

~~~rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
~~~

* 결과값

~~~
224
164
168
224
// ... etc
~~~

