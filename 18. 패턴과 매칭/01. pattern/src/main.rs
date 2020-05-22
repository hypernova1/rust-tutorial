/*
    패턴이 사용될 수 있는 모든 곳
*/

fn main() {
    _match();
    if_let();
    while_let();
    _for();

    let point = (3, 5);
    print_coordinates(&point);
}

/*
    match
        - 모든 경우의 수를 빠짐없이 작성해야함
*/
fn _match() {

    let num = 2;

    match num {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("{}", num), // _: 모든 패턴에 매칭되기 때문에 마지막 갈래에 쓰여 나머지를 표현함 => 명시하지 않은 값들을 무시할 때 유용
    }
}

/*
    if let
        - match를 더 짧게 표현
        - 여러 패턴에 하나만 대응시키는 match 표현 보다 유연하게 표현 가능
        - 모든 패턴을 다루는 것을 판단하지 않기 때문에 처리되지 않은 경우에 따른 버그를 사전에 경고해주지 않음
*/
fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background bolor");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/*
    while let
*/
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/*
    for
*/
fn _for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() { //enumerate: 튜플로 만들어서 반환해줌
        println!("{} is at index {}", value, index);
    }
}

/*
    let
        - 형식: let PATTERN = EXPRESSION;
*/
fn _let() {
    let x = 5;

    let (x, y, z) = (1, 2, 3); // 디스트럭쳐링: 튜플과 원소의 개수가 맞지 않으면 컴파일 에러
}

/*
    함수의 매개변수
*/
fn foo(x: i32) {}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}