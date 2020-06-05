/*
    테스트 조직화

    단위 테스트
        - 각 코드의 단위를 나머지 부분과 분리하여 테스트
        - 어느 부분이 기대한대로 동작하지 않는지를 빠르게 찾을 수 있도록 하기 위함
*/

/*
    [#cfg(test)]
        - cgf: configuration
        - cargo test를 실행시킬 때에만 컴파일
*/

pub fn add_two(a: i32) -> i32 {
    internal_addr(a, 2)
}

fn internal_addr(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    //비공개 함수 테스트
    #[test]
    fn internal() {
        assert_eq!(4, internal_addr(2, 2));
    }


}

