/*
    고급 함수와 클로저
*/

/*
    함수 포인터
*/
fn add_one(x: i32) -> i32 {
    x + 1
}

/*
    fn 타입을 사용하여 함수 포인터를 인자로 사용하기
        - fn은 트레잇이 아니고 타입임
        - fn을 트레잇 바운드로 Fn 트레잇 중 하나를 사용한 제네릭 타입 파라미터를 정의하기 보다는 직접 파라미터 타입으로 특정
        - 함수 포인터는 클로저 트레잇 세종류(Fn, FnMut, FnOnce)를 모두 구현하므로, 클로저를 인자로 받는 함수에게 넘길 수 있음
 */
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/*
    클로저 반환하기
        - 클로저는 직접 반환할 수 없음
        - 크기를 알 수 없기 때문
*/
//컴파일 에러
// fn return_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

fn return_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);


    // map의 인자로 클로저를 사용하여 숫자 벡터를 스트링 벡터로 전환
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("{:?}", list_of_string);

    // 클로저 대신 map의 인자로 함수 사용
    let list_of_string: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) //ToString 트레잇 내에 정의된 to_string함수 사용
        .collect();

    println!("{:?}", list_of_string);

}
