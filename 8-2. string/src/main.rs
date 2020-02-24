/*
    스트링
        - 러스트 표준 라이브러리를 통해 제공
        - 가변적
        - 소유권을 가짐
        - UTF-8로 인코딩된 스트링 타입
*/
fn main() {

    /*
        스트링 생성 방법
    */
    let mut s = String::new();

    let mut s = String::from("initial contents");

    let data = "initial contents";

    let s = data.to_string();   // String::from("initial contents") 와 같음

    let s = "initial content".to_string();



    /*
        스트링 갱신하기
    */
    let mut s = String::from("foo");
    s.push_str("bar");

    /*
        + 연산자나 format! 매크로를 이용한 접합

        + 연산자는 add 메소드를 사용함 -> fn add(self, s: &str) -> String {
    */
    let s1 = String::from("Hello,");
    let s2 = String::from(",world!");
    let s3 = s1 + &s2; // s1은 이동되어 더이상 사용할 수 없음

    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // 매크로를 사용하여 문자열 합치기 (소유권을 가져가지 않음)

    /*
        러스트는 String의 인덱싱을 지원하지 않음
            - 각각의 유니코드의 스칼라 값이 다 다르기 때문에
    */


    /*
        스트링 슬라이싱
    */
    let hello = "hello";
    let s = &hello[0..3];

    println!("{}", s);

    /*
        스트링 내에서 반복적으로 실행되는 메소드
    */
    for c in "hello".chars() {
        println!("{}", c);
    }
    for b in "hello".bytes() {
        println!("{}", b);
    }

}
