/*
    match: 흐름 제어 연산자
*/
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/*
    값들을 바인딩하는 패턴들
*/
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 4,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

/*
    Option 을 이용한 매소드
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin1: Coin = Coin::Quarter(UsState::Alabama);
    let coin2: Coin = Coin::Penny;

    let return_value = value_in_cents(coin1);
    let return_value2 = value_in_cents(coin2);

    println!("{}", return_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /*
        변경자 (placeholder)
    */

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 이외의 다른 값들
    }

}
