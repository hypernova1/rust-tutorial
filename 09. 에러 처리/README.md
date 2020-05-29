# 에러 처리

[1. panic!](#1-panic!)  
[2. Result](#2-Result)  

## 1. `panic!`
* 복구 불가능한 에러
> ### `panic!`을 사용하여 스택을 되감거나 그만두기  
> * 기본적으로 `panic!`이 발생하면 되감기(*unwinding*)를 시작하여
> 패닉이 발생한 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거함
> * 되감기는 시간이 걸리기 때문에 대안으로 그만두기(*abort*)를 사용가능
>   * 데이터 제거 없이 프로그램 종료
>   * Cargo.toml내에서 `[profile]` 섹션에 `panic = 'abort'`를 추가  
> * 릴리즈 모드에서 그만두기를 사용하고 싶다면 다음을 추가
> ~~~
> [profile.release]
> panic = 'abort'
> ~~~
 
### 프로그램 내에서 `panic!` 호출
~~~rust
fn main() {
  panic!("crash and burn");
}
~~~

#### 실행 결과
~~~
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
~~~

### `panic!` 백트레이스 사용
#### 벡터의 길이를 초과하는 요소 접근 시도
~~~rust
fn main() {
  let v = vec![1, 2, 3];

  v[99]; //buffer overread!!
}
~~~
#### 실행결과
~~~
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', /stable-dist-rustc/build/src/libcollections/vec.rs:1362
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
~~~
* *libcollections/vec.rs*에서 `panic!` 발생

#### `RUST_BACKTRACT` 설정시
~~~
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', /stable-dist-rustc/build/src/libcollections/vec.rs:1392
stack backtrace:
   1:     0x560ed90ec04c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /stable-dist-rustc/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x560ed90ee03e - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:351
   3:     0x560ed90edc44 - std::panicking::default_hook::h1670459d2f3f8843
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:367
   4:     0x560ed90ee41b - std::panicking::rust_panic_with_hook::hcf0ddb069e7abcd7
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:555
   5:     0x560ed90ee2b4 - std::panicking::begin_panic::hd6eb68e27bdf6140
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:517
   6:     0x560ed90ee1d9 - std::panicking::begin_panic_fmt::abcd5965948b877f8
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:501
   7:     0x560ed90ee167 - rust_begin_unwind
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:477
   8:     0x560ed911401d - core::panicking::panic_fmt::hc0f6d7b2c300cdd9
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:69
   9:     0x560ed9113fc8 - core::panicking::panic_bounds_check::h02a4af86d01b3e96
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:56
  10:     0x560ed90e71c5 - <collections::vec::Vec<T> as core::ops::Index<usize>>::index::h98abcd4e2a74c41
                        at /stable-dist-rustc/build/src/libcollections/vec.rs:1392
  11:     0x560ed90e727a - panic::main::h5d6b77c20526bc35
                        at /home/you/projects/panic/src/main.rs:4
  12:     0x560ed90f5d6a - __rust_maybe_catch_panic
                        at /stable-dist-rustc/build/src/libpanic_unwind/lib.rs:98
  13:     0x560ed90ee926 - std::rt::lang_start::hd7c880a37a646e81
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:436
                        at /stable-dist-rustc/build/src/libstd/panic.rs:361
                        at /stable-dist-rustc/build/src/libstd/rt.rs:57
  14:     0x560ed90e7302 - main
  15:     0x7f0d53f16400 - __libc_start_main
  16:     0x560ed90e6659 - _start
  17:                0x0 - <unknown>
~~~
* 이러한 정보들과 백트레이스를 얻기 위해서는 디버그 심볼이 활성화 되어있어야 함
* `cargo build`, `cargo run`을 `--release` 플래그 없이 실행시 기본으로 활성화


## 2. `Result`
* 복구 가능한 에러

#### 표준 라이브러리에 정의된 `Result`
~~~rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
~~~
* `T`: 성공한 경우에 `Ok` 변수 내에 반환될 값의 타입
* `E`: 실패한 경우에 `Err` 변수 내에 반환될 에러의 타입

### I. `Result` 사용하기
#### 파일 열기
~~~rust
use std::fs::File;

fn main() {
  let f: u32 = File::open("hello.txt");
}
~~~

#### 컴파일시
~~~
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
~~~
* `File::open`함수의 반환값이 `Result<T, E>`타입으로 예상되지만 핸들링하지 않아 패닉이 발생
  * `T`: `std::fs::File`
  * `E`: `std::io::Error`

#### `Result` 핸들링
~~~rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {}", error)
    },
  }
}
~~~

#### 파일이 없을시 생기는 `panic!`
~~~
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
~~~


### II. 서로 다른 에러 매칭하기
* `File::open`의 실패 이유에 따라 행동 정의

~~~rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(ref error) if error.kind() == ErrorKind::NotFound => {
      match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => {
          panic!("Tried to create file but there was a problem: {:?}", e)
        },
      }
    },
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    },
  };
}
~~~
* 에러가 발생하면 `io::ErrorKind` 열거형을 얻어서 에러 종류마다 정의
* `if error.kind() == ErrorKind::NotFound`: 매치가드(*match guard*)

### III. `unwrap`, `expect`
* 에러가 났을 때 패닉을 위한 숏컷

~~~rust
use std::fs::File;

fn main() {

   //미리 정의된 에러 메세지 호출
  let f = File::open("hello.txt").unwrap();
  //사용자 정의 에러 메세지
  let f = File::open("hello.txt").expect("Failed to open hello.txt"); 
}
~~~

### IV. 에러 전파하기
* 함수 내에서 에러를 처리하지 않고 호출하는 코드쪽으로 결과를 반환하여 그쪽에서 처리하도록함

~~~rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  }

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
~~~

#### `?`: 에러를 전파하기 위한 숏컷
* `Result`타입을 반환하는 함수에서만 사용 가능
~~~rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {

  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)

}
~~~
* `Ok`가 발생하면 변수에 값을 f에 반환하고 에러가 발생하면 호출하는 코드에 `Err` 반환

#### chaning을 통해 코드 더 줄이기
~~~rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s);
}
~~~