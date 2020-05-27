/*
    고급 타입


    타입 안정성과 추상화를 위한 뉴타입 패턴 사용하기
        - 어떤 값이 혼동되지 않도록 정적으로 강제하는 것과 어떤 값의 단위로서의 기능 포함
        - 어떤 타입의 몇몇 자세한 구현 사항을 추상화
*/

/*
    타입 별칭
*/

fn main() {
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;

    // 별칭은 이름만 다를 뿐 원래 타입과 동일하여 연산 가능
    println!("x + y = {}", x + y);


    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    
    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     //...
    // }
    
    // fn return_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     //...
    // }
    
    // // 별칭을 사용하여 코드 줄이기
    // type Thunk = Box<dyn Fn() + Send + 'static>;

    // let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type2(f: Thunk) {
    //     //...
    // }

    // fn returns_long_type2() -> Thunk {
    //     //...
    // }

    // !타입을 갖는 loop
    println!("forever ");
    loop { //루프가 결코 끝나지 않으므로 !가 표현식임
        println!("and ever ");

        break; //break를 포함시키면 거짓이 되기 때문에 loop를 빠져나감
    }
}

/*
    부정타입 (!)
*/
fn bar() -> ! {
    let guess = "1";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //continue는 !값을 가짐
        };

        println!("{}", guess);
    }
}

// panic! 매크로에서 사용
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

/*
    동적인 크기의 타입과 Sized
        - 하나의 타입은 모두 동일한 크기의 메모리를 사용해야함
*/

fn str() {
    // 동작하지 않는 코드들
    // let s1: str = "Hello there!"; //12바이트
    // let s1: str = "How's it going?"; //15바이트

    // 스트링 슬라이스는 슬라이스의 시작 위치와 길이를 저장
    // 따라서 &str은 두개의 값을 가지고 있음
    let s1: &str = "Hello there!";
    let s1: &str = "How's it going?";
}

//트레잇은 동적인 크기의 타입
fn generic<T> (t: T) {

}

//위의 함수는 실제로 아래와 같이 컴파일 됨
fn generic_compile<T: Sized>(t: T) {

}

//?Sized: T가 Sized일 수도 있고 아닐 수도 있음 오직 ?는 Sized에서만 사용 가능
fn generic2<T: ?Sized>(t: &T) { //소유권이 아닌 참조자를 가져옴

}