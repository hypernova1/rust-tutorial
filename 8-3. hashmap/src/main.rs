/*
    해쉬맵
        - key 와 value 로 이루어져 있는 컬렉션
        - 다른 컬렉션보다 덜 사용이 되기 때문에 프렐루드(prelude) 내에 자동으로 가져와지는 기능이 없음
        - 표준 라이브러리로부터 지원을 덜 받음 (ex. 매크로가 없음)
*/

use std::collections::HashMap;

fn main() {

    // 선언
    let mut scores = HashMap::new();
    // 값 넣기
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // 벡터 두개로 해쉬맵 만들기
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /*
        해쉬맵과 소유권
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 키와 값에 삽입되는 순간 소유권은 해쉬맵에게 넘어간다
    map.insert(field_name, field_value);

    // println!("{}, {}", field_name, field_value); // 더 이상 유효하지 않음


    /*
        값 접근하기
    */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Option<&V>를 반환

    /*
        루프 돌기
    */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /*
        갱신하기
    */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 덮어써짐

    println!("{:?}", scores);

    /*
        키에 할당된 값이 없을 경우에만 삽입
    */
    scores.entry(String::from("Blue")).or_insert(70);
    scores.entry(String::from("Green")).or_insert(30);
    println!("{:?}", scores);


    /*
        예전 값을 기초로 값을 갱신하기
    */
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
