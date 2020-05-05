/*
    복구 불가능한 에러

    러스트가 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가며 데이터를 제거
    
    그만두기(abort) 데이터 제거 없이 프로그램 종료
        - Cargo.toml의 [profile] 섹션에 panic = 'abort' 추가
*/

fn main() {

    //panic!("crash and burn");

    /*
        panic! 백트레이스 사용
    */
    let v = vec![1, 2, 3];

    v[99]; //버퍼 오버리드(buffer overread)

}
