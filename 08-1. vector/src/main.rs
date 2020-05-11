/*
    벡터
        - 단일 구조 안에 하나 이상의 값을 저장
        - 같은 타입의 값만 저장 가능
*/
fn main() {

    let v: Vec<i32> = Vec::new(); // 함수를 사용하여 생성

    let v = vec![1, 2, 3]; // 매크로를 사용하여 생성

    /*
        벡터 갱신하기
    */
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];
        // v 사용
    } // v가 스코프 밖으로 벗어났으므료 데이터 해제

    /*
        벡터의 요소들 읽기
    */

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    /*
        유효하지 않은 참조자 (참조자 규칙)
            - 아이템에 대한 참조자를 가지는 동안 벡터에 요소 추가 시도하기
    */
    let mut x = vec![1, 2, 3, 4, 5];
    let first = &x[0];
    x.push(6); // error
    // println!("{}", first);


    /*
        백터 이터레이터
    */
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // *: 역참조 연산자 (가변 참조자가 참고하는 값을 바꾸기 위해서)
    }

    /*
        열거형을 사용하여 여러타입 저장
    */
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(2.3),
        SpreadsheetCell::Text(String::from("hello")),
    ];

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


