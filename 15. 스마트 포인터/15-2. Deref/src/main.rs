/*
    Deref 트레잇
        - 트레잇으로 스마트 포인터를 참조자같이 취급
        - 역참조 연산자(*)의 동작을 커스터마이징

    DerefMut 트레잇
        - 가변 참조자에 대한 *를 오버라이딩
        - 다음의 세가지 경우에 대항하는 타입과 트레잇 구현을 찾았을 때 역참조 강제를 수행
            - T: Deref<Target=U>일때 &T에서 &U로                 불변에서 불변으로
            - T: DerefMut<Target=U>일 때 &mut T에서 &mut U로     가변에서 가변으로
            - T: Deref<Target=U>일 때 &mut T에서 &U로            가변에서 불변으로
            - 불변에서 가변으로는 허용되지 않음
*/

use std::ops::Deref;

fn main() {

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // *를 사용하여 포인터 주소에 있는 값을 얻기


    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // Box<T>를 참조자처럼 사용
    assert_eq!(5, *y);

    let my_box = MyBox(3);

    println!("{}", *my_box); // *my_box는 *(my_box.deref())와 같음

    /*
        함수와 메소드를 이용한 암묵적 역참조 강제
            - Deref를 구한한 타입의 참조자를 Deref가 본래의 타입으로 바꿀 수 있는 타입의 참조자로 바꿈
            - 특정한 참조자를 함수나 메소드의 인자로 넘기는 중 타입이 맞지 않을 때 발생
     */

    let name = MyBox::new(String::from("sam"));

    hello(&name);

    hello(&(*name)[..]); //역참조 강제가 없었다면..

}

// 스마트 포인터 만들기
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Deref 트레잇이 사용할 연관 타입 정의
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}