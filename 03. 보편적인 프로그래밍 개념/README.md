# 3. 보편적인 프로그래밍 개념

[1. 변수와 가변성](#1-변수와-가변성)  
[2. 데이터 타입들](#2-데이터-타입들)  
[3. 함수 동작 원리](#3-함수-동작-원리)  
[4. 주석](#4-주석)  
[5. 제어문](#5-제어문)  

## 1. 변수와 가변성
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

## 2. 데이터 타입들
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

## 3. 함수 동작 원리
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


## 4. 주석

## 5. 제어문
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