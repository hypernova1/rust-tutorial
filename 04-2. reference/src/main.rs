fn main() {
    /*
        참조자(References)와 빌림(Borrowing)

        참조자의 규칙
            1. 아래의 2가지의 경우를 한 번에 가질 수 없음
                - 하나의 가변 참조자
                - 임의 개수의 불변 참조자들
            2. 참조자는 항상 유요해야만 함

    */
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // &(참조자)를 넘김

    chagne(&mut s1);

    println!("The length of '{}' is {}.", s1, len); // 유효


    /*
        참조자 제한 사항 (data race)
            1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다
            2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
            3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.
    */
    let r1 = &mut s1;
    let r2 = &mut s1;
    // println!("{}", r1); // error

    /*
        해결 방법
    */
    let mut s2 = String::from("hello");
    {
        let r3 = &mut s2;
    } // 여기서 r3가 스코프 밖으로 벗어났기 때문에 새로운 참조자 생성 가능
    let r4 = &mut s2;
    println!("{}", r4);


    let mut s3 = String::from("hello");
    let r4 = &s3; // 문제 없음
    let r5 = &s3; // 문제 없음
    let r6 = &mut s3; // 큰 문제



    /*
        댕글링 참조자(Dangling References
            - 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터
    */
    // let reference_to_nothing = dangling();
    let reference_to_nothing = no_dangling();



}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s는 스코프 밖으로 벗어났지만 가리키고 있는 값에 대한 소유권이 없어서 아무일도 발생하지 않음

fn chagne(some_string: &mut String) { // & mut : 가변 참조자(Mutable References)
    some_string.push_str(", world");
}

// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s
// } // 함수의 반환 타입은 참조자를 포함하는데 실제로 참조하는 값은 없음

fn no_dangling() -> String {
    let s = String::from("hello");
    s
}