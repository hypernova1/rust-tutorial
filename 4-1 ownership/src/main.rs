fn main() {

    /*
        소유권
            - 기존의 가비지 컬렉션을 갖고 있는 언어나 직접 메모리를 해제해 주어야 하는 언어와 다른 제 3의 접근법
            - 메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 서유권 시스템을 통해 관리됨
            - 런타임 비용이 발생하지 않음
    */

    /*  규칙
            1. 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 가짐
            2. 한 번에 하나의 오너만 존재할 수 있음
            3. 오너가 스코프 밖으로 벗어나면, 값은 버려짐(dropped)
    */



    /*
        변수의 스코프
    */
    {
        let s = "hello"; // 이 지점부터 유효
    }



    /*
        String 타입
        스트링 리터럴은 불변이지만 String::from 으로 감싸주면 변경 가능해짐
    */
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{}", s2);

    /*
        메모리와 할당
            - String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어짐
            - 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있음
                =>
                1. 런타임에 운영체제로부터 메모리가 요청되어야함. ex. String::from("~~")
                2. String 의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요 => 스코프를 벗어나는 순간 자동 반납
                    (중괄호가 닫힐 때 자동적으로 drop 이라는 함수를 호출
    */


    /*
        이동(move): 변수와 데이터가 상호작용하는 방법
    */
    let s3 = String::from("hello");
    let s4 = s3; // s3의 데이터가 s4로 이동이 됨 (s3은 비어있는 변수가 됨)
    /*
        s3에 있는 값도 보존하고 싶다면 clone 메소드를 호출해야 함
    */


    /*
        복사(Copy): 스택에만 있는 데이터
        해당 타입들
            1. u32와 같은 모든 정수형 타입
            2. boolean
            3. f64와 같은 부동 소수점 타입
            4. Copy 가 가능한 타입만으로 구성된 튜플들, ex) (i32, i32) O / (i32, String) X
    */
    let x = 5;
    let y = x; // x, y 모두 5가 할당됨



    /*
        소유권과 함수
    */

    let s5 = String::from("hello");
    takes_ownership(s5); // 더 이상 s5가 유효하지 않음
    // println!(s5); //컴파일 에러

    let z = 5;
    makes_copy(z);
    println!("{}", z); // 사용 가능



    /*
        반환 값과 스코프
            - 값의 반환 또한 소유권을 이동시킴
    */
    let str1 = gives_ownership(); // 반환값을 str1 에게 이동시킴

    let str2 = String::from("hello");

    let str3 = takes_and_gives_back(str2); // str2는 함수안으로 이동되었고, 함수가 반환값을 str3  로 이동
}
/*
    str1, str3 는 스코프 밖으로 벗어나 drop 이 호출됨
    str2는 스코프를 벗어낫지만 이동되었으므로 아무일도 일어나지 않음
*/


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// some_string 은 스코프 밖으로 벗어나고 drop 이 호출됨

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 반환 후 호출한 쪽의 함수로 이동
}