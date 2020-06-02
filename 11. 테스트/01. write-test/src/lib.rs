#[cfg(test)]
mod tests {
    use super::*; //외부 모듈 가져오기

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }


    //실패하는 테스트 만들기
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    /*
        동치 테스트
            - 비교되는 값들은 PartialEq 와 Debug 트레잇을 구현해야 함
     */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // ==
        assert_ne!(5, add_two(2)); // !=
    }

    // 실패 메세지 커스텀
    #[test]
    fn greeting_contains_name() {
        let result = greeting("sam");
        assert!(
            result.contains("sam"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    //should_panic 을 이용하여 어떤 조건이 panic! 을 일으키는지 테스트
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    format!("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}