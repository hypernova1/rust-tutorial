/*
    함수 결과 보여주기
*/

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}


#[cfg(test)]
mod tests {
    use super::*;

    /*
        두 개의 테스트 실행시 성공한 테스트의 결과는 볼 수 없음
        성공하는 테스트 결과를 보고 싶다면 cargo test -- --nocapture 명령어를 이용하여 출력 캡처 동작을 비활설화 할 수 있음
    */
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, 10);
    }


    /*
        이름으로 테스트의 일부분만 실행
            - cargo test <실행할 함수의 이름>
                - 인자로 넘긴 이름이 포함되어 있는 모든 함수 실행
    */
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }


    /*
        테스트 무시하기
            - #[ignore] 추가시 함수에서 제외
            - 무시된 테스트만 실행시키고 싶다면 cargo test -- --ignored
    */
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests2 {
    use super::*;

   
}

