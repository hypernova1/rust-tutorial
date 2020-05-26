/*
    고급 트레잇

    
    기본 제네릭 타입 파라미터와 연산자 오버라이딩
        - 러스트는 연산자를 임의로 만들거나 오버로딩하는 것을 허용하지 않음
        - 하지만 std::ops에 나열되어 있는 연산자와 연관된 구현ㅇ르 하는 것으로 연산자 및 관련된 트레잇을 오버라이딩 가능
*/
use std::ops::Add;
use std::fmt;


#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}

/*
    Add 트레잇의 정의

    trait Add<RHS=Self> { //RHS=Self: 기본 타입 파라미터, add메소드의 rhs 파라미터 타입을 정의
        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
    }
*/

/*
    모호성 방지를 위한 완전 정규화 문법: 동일한 이름의 메소드 호출하기
        - 러스트는 서로 다른 트레잇의 메소드 이름이 동일한 것을 방지할 수 없음
        - 두 트레잇을 모두 한 타입에 대해 구현하는 것을 방지할 방법도 없음
        - 어떤 타입에 대해 트레잇의 메소드와 동일한 이름을 가진 메소드를 직접 구현하는 것도 가능
*/
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captin speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


/*
    서로 다른 객체의 연관함수 호출
*/
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
/*
    완전 정규화 문법의 정의
        - <Type as Trait>::function(receiver_if_method, next_arg, ...);
        - 연관 함수에서는 receiver가 없고 다른 인자들의 리스트만 존재
        - 이 문법에서 러스트가 프로그램 내의 다른 정보로 부터 알아낼 수 있는 부분은 생략이 허용됨
*/


/*
    슈퍼트레잇을 사용하여 어떤 트레잇 내에서 다른 트레잇의 기능 요구하기

    **********
    *        *
    * (1, 3) * 그리기
    *        *
    **********
*/
trait OutlinePrint: fmt::Display { //Display를 구현한 타입은 to_string() 함수 사용 가능
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point { //Display가 구현되지 않으면 OutputlinePoint를 구현할 수 없음
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

/*
    외부 타입에 대해 외부 트레잇을 구현하기 위한 뉴타입 패턴
        - 뉴타입이란 하스켈 프로그래밍 언어로부터 기원된 용어
        - 런타임 성능 패널티는 없으며 래퍼 타입은 컴파일시 생략됨
        - 트레잇을 구현하려면 타입 혹은 트레잇 둘 중 최소 하나는 해당 크레이트 내의 것이어야함
        - 뉴타입 패턴을 사용하면 이 제약을 우회할 수 있음 -> 튜플 구조체에 새로운 타입을 만들기
        - Wrapper가 새로운 타입이므로, 들고 있는 원래 값의 메소드를 가지지 못한다는 단점이 있음
            - 새로운 타입이 내부 타입이 가지고 있는 메소드를 갖길 원한다면 Wrapper상에서 Deref 트레잇을 구현
*/
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); //Human에 직접 구현된 fly 메소드 호출


    println!("A baby dog is called a {}", Dog::baby_name()); //Dog에 직접 정의된 연관함수 호출
    // println!("A baby dog is called a {}", Animal::baby_name()); // 어떤 구현체를 사용하는지 알지 못하기 때문에 컴파일 에러
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //완전 정규화 문법을 사용하여 함수 호출


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

