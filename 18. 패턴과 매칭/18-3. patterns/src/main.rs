/*
    패턴 문법의 모든 것
*/

/*
    리터럴 매칭
*/
fn literal_matching() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/*
    명명 변수 매칭
        - 반증 불가 패턴
*/
fn naming() {

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Get 50"),
        Some(y) => println!("matched, y = {:?}", y),
        _ => println!("Default case x = {:?}, y = {:?}", x, y),
    }

}

/*
    다중 패턴
        - | (or) 구문을 사용하여 여러개의 패턴과 매칭
*/
fn multiple_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/*
    ..=을 이용한 값의 범위 매칭
*/
fn range_matching() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"), //숫자 범위 매칭
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"), //캐릭터 범위 매칭
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/*
    값을 해체하여 분리하기
*/
// 구조체 해체하기
struct Point {
    x: i32,
    y: i32,
}

fn struct_destructuring() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p; //약칭 구문

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x ,y),
    }
}

// 열거형 해체
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_destructuring() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y,
            );
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            )
        }

    }
}

// 참조자 해체
fn reference_destructuring() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point {x, y}| x * x + y * y)
        .sum();

    println!("{:?}", sum_of_squares);
}

// 구조체와 튜플 해체
fn struct_and_tuple_destructuring() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

/*
    패턴 내에서 값 무시하기
*/

// _를 이용하여 전체 값 무시하기
fn ignore(_: i32, y: i32) {
    println!("This code only uses the y parameter {}", y);
}

// 중첩된 _를 이용하여 값의 일부 무시하기
fn ignore2() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
    println!("setting is {:?}", new_setting_value);


    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}

// 언더스코어로 시작하는 이름을 사용하여 쓰이지 않는 변수 무시
fn underscore_ignore() {
    let _x = 5; // 컴파일러가 경고를 생성하지 않음
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s { //s의 소유권을 가져가지 않음
        println!("found a string");

    }

    println!("{:?}", s);
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}
// ..을 이용하여 값의 나머지 부분 무시하기
fn ignore3() {
    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("first is {}, last is {}", first, last),
        // (.., second, ..) => println!("first is {}, last is {}", first, last), // second 전후에 몇개씩 둘지 결정할 수 없기 때문에 컴파일 에러

    }
}

/*
    ref와 ref mut을 이용하여 패턴 내에서 참조자 생성
*/
fn ref_and_ref_mut() {
    let robot_name = Some(String::from("Bors"));

    // match robot_name {
    //     Some(name) => println!("Found a name: {}", name),
    //     None => (),
    // }
    // println!("robot_name is {:?}", robot_name); //소유권이 이동했기 때문에 컴파일 에러

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name), //ref를 사용하여 참조자만 가져옴
        None => (),
    }

    println!("robot_name is {:?}", robot_name); //ok

    
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"), //역참조를 사용하여 값 변경
        None => (),
    }

    println!("robot_name is {:?}", robot_name);
}

/*
    매치 가드를 이용한 추가 조건
        - match 갈래 뒤에 if문이 추가로 붙음
*/
fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_guard2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/*
    @ 바인딩: at 연산자
        - 해당 값이 패턴과 매치되는 지 확인 + 해당 값을 갖는 변수 생성
*/
fn at() {
    enum Message2 {
        Hello { id: i32 }
    }

    let msg = Message2::Hello{ id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id range {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}


fn main() {
    literal_matching();
    naming();
    multiple_pattern();
    range_matching();
    struct_destructuring();
    enum_destructuring();
    reference_destructuring();
    struct_and_tuple_destructuring();
    ignore(0, 3);
    ignore2();
    underscore_ignore();
    ignore3();
    ref_and_ref_mut();
    match_guard();
    match_guard2();
    at();
}
