/*
    Rc<T> (reference counting): 참조 카운팅 스마트 포인터
        - 하나의 값이 여러개의 소유자를 가져야할 때 사용
        - 참조자의 갯수를 추적
        - 데이터를 힙에 할당하고 어떤 부분에서 그 데이터를 마지막에 이용하게 될지 컴파일 타임에 알 수 없는 경우 사용
        - 단일 스레드에서만 사용 가능
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); //참조 카운트 출력
    let b = Cons(3, Rc::clone(&a)); //Rc::clone(&a)는 a.clone()과 다르게 얕은 복사를 함
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after creating a = {}", Rc::strong_count(&a));

}
