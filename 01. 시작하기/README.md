# 시작하기

[1. 설치하기](#1-설치하기)  
[2. Hello, World!](#2-hello-world)  
[3. Hello, Cargo!](#3-hello-cargo)  

## 1. 설치하기

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

## 2. Hello, World!

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

## 3. Hello, Cargo!

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
