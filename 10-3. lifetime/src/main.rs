/*
    라이프타임
        - 해당 참조자가 유효한 스코프
        - 기본적으로는 암묵적이며 추론됨
        - 댕글링 참조자를 방지
*/

use std::fmt::Display;

// 함수에서의 제네릭 라이프 타임
//     - 반환되는 참조자가 x인지 y인지 모르기 때문에 라이프타임을 지정해서 빌림검사기가 분석할 수 있게 함
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// y는 반환값과 관계가 없기 때문에 라이프타임을 명시하지 않음
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{}", y);
    x
}

// 구조체 정의 상에서 라이프타임 명시
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 빌림검사기: 모든 빌림이 유효한지를 결정하귀 위해 스코프를 비교
    let r;          // ------+-- 'a
                          //       |
    {                     //       |
        let x = 5;   // -+----+-- 'b
        r = &x;           //  |    |
    }                     // -+    |
                          //       |
    // println!("{}", r); //       |
                          // ------+

    let s1 = "hello";
    let s2 = "hello, world";

    let result = longest(s1, s2);

    println!("{}", result);

    longest2(s1, s2);

    let novel = String::from("Call me Ishmael. Some years ago..");
    let first_sentence = novel.split(".")
        .next()
        .unwrap();

    let i = ImportantExcerpt { part: first_sentence };

    println!("{}", i.part);
}

/*
라이프타임 생략 규칙
- 다음의 규칙들이 맞지 않으면 컴파일 에러
    * 입력 라이프타임(함수나 메소드의 파라미터에 대한 라이프타임)
        1. 참조자인 각각의 파라미터는 고유한 파라미터를 가짐
        * 예
            - 하나의 라이프타임: fn foo<'a>(x: &'a i32)
            - 두개의 라이프타임: fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    * 출력 라이프타임(반환 값에 대한 라이프타임)
        2. 단 하나의 라이프타임 파라미터만 있는 경우, 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입
            - 예: fn foo<'a>(x: &'a i32) -> &'a i32
        3. 여러 개의 입력 라이프타임 파라미터 중 하나가 &self나 &mut self이면 self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입됨
*/

/*
    fn first_word(s: &str) -> &str {}
    첫 번째 규칙 적용
    -> fn first_word_compile<'a>(s: &'a str) -> &str {}
    두 번째 규칙 적용
    -> fn first_word_compile<'a>(s: &'a str) -> &'a str {}
*/

/*
    fn longest(x: &str, y: &str) -> &str {}
    첫 번째 규칙 적용
    -> fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {} // 에러 발생
*/


/*
    메소드 정의 내에서의 라이프타임 명시
*/
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // 첫 번째 생략 규칙이 적용 되므로 라이프타임을 명시하지 않음
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str { // 세 번째 생략 규칙 적용
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
    정적 라이프타임
        - 프로그램의 전체 생명주기를 가리킴
        - 프로그램의 바이너리 내에 직접 저장되며 항상 이용 가능
        - 모든 스트링 리터럴의 라이프타임임

        let s: &'static str = "I have a static lifetime";
*/

/*
    제네릭과 트레잇 바운드, 라이프타임을 함께 사용
*/

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
