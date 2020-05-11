/*
    mod(모듈)와 파일시스템
        생성 방법: cargo new communicator (--bin 제거)

        pub: 외부 파일에서 사용 가능 (rust 의 모든 코드는 기본적으로 private 임)

    모듈 파일 시스템의 규칙
        - foo 라는 이름의 모듈이 서브 모듈을 가지고 있다면 foo.rs 파일 내에 foo 에 대한 선언을 넣어야 함
        - foo 가 서브 모듈을 가지고 있다면 foo/network 안에 foo 에 대한 선언을 넣어야 함


    비공개 규칙(Private Rules)
    1. pub 이면 부모 모듈 어디서든 접근 가능
    2. 비공개면 같은 파일 내에 있는 부모 모듈 및 부모의 자식 모듈에서만 접근 가능
*/
pub mod network; // network.rs 와 연결

pub mod client; // client.rs 와 연결



/*
    super 를 사용하여 부모 모듈에 접근하기
*/
#[cfg(test)]
mod tests {

    use super::client;

    #[test]
    fn it_works() {
        // super::client::connect(); // tests 모듈은 절대 경로가 필요한 듯
        client::connect();
    }
}
