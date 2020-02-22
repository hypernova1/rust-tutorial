fn main() {

    /*
        슬라이스(Slices)
            - 소유권을 갖지 않는 데이터 타입
            - 컬렉션(Collection) 전체가 아닌 컬렉션의 연속된 일련의 요소 참조 가능
    */

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // String 을 ""로 만듬

    println!("{}", word); // String 은 빈 값이 되었지만 word 값은 여전히 유효함


    /*
        스트링 슬라이스
    */

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", world);


    /*
        해결 방법
    */
    let mut s2 = String::from("hello world");
    let word2 = new_first_word(&s2);
    println!("{}", word2);

    s2.clear();
    println!("is empty {}", s2);

    //println!("{}", word2); // word 가 유효하지 않음


    /*
        스트링 리터럴은 슬라이스이다.
    */
    let s3 = "Hello, world"; // &str (불변 참조자)

    let word = new_first_word2(&s3[..]);
    let word = new_first_word2(s3); // 위와 같음


    /*
        그 밖의 슬라이스들
    */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn new_first_word(s: &String) -> &str { // &str : 스트링 슬라이스
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn new_first_word2(s: &str) -> &str { // &str : 스트링 슬라이스
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
