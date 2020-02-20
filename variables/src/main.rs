fn main() {
    let mut x = 5; // let 키워드를 쓸때 mut(mutable)을 붙이지 않으면 재할당이 불가능.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1; // Shadowing: 값의 유형을 변경할 수도 있음 (mut 키워드와의 차이점)
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = space.len(); // 다른 자료형이 들어갈 수 있음.

    /*
    let mut spaces = "     ";
    spaces = spaces.len(); // 컴파일 에러
    */

    /*
        데이터 타입들
        스칼라: 하나의 값으로 표현되는 타입(정수형, 부동 소수점 숫자, boolean ,문자)
    */

    /*
        정수형
        Length  signed  unsigned
        8-bit   i8      u8
        16-bit  i16     u16
        32-bit  i32     u32
        64-bit  i64     u64
        arch    isize   usize  >  운영체제 비트수에 따라 다름
    */

    /*
        부동 소수점 타입
    */
    let z = 2.0; // f64
    let a: f32 = 3.0; // f32



    /*
        복합 타입: 다른 타입의 다양한 값들을 하나의 타입으로 묶음(튜플 배열)
    */
    /*
        튜플: 모든 요소가 타입이 달라도 됨
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (b, c, d) = tup; // 튜플 디스트럭쳐링
    println!("The value of b is: {}", b);

    let e = tup.0; // 튜플 인덱스로 할당
    let f = tup.1;

    /*
        배열
            1. 모든 요소의 타입이 같아야함
            2. 길이는 고정됨
    */
    let month = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let september = month[8];
}
