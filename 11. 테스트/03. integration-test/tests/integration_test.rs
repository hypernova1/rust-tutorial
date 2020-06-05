/*
    통합 테스트
        - 라이브러리의 파트들이 함께 올바르게 작동하는지 테스트
        - tests 디렉토리 내에 작성
        - 특정 통합 테스트 실행 커맨드: cargo test --test <test file name>
*/

extern crate adder;

//서브 테스트 모듈 가져오기
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

