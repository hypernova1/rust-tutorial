# 컬렉션(Collections)

* [1. 벡터](#1-벡터(Vec<T>))
* [2. 스트링](#2-스트링)
* [3. 해쉬맵](#3-해쉬맵)

## 1. 벡터(`Vec<T>`)
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

## 2. 스트링
* 스트링 슬라이스(`str`)와 다르게 `String`타입은 표준 라이브러리를 통해 제공됨
* UTF-8 인코딩

### I. 생성하기
~~~rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string(); //String::from("initial contents")와 같음

let s = "initial contents".to_string();
~~~

### II. 갱신하기
#### II.i `push_str`, `push`
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

#### II.ii `+`연산자나 `format!`매크로를 이용한 접합
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

#### II.iii 스트링 내부의 인덱싱

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