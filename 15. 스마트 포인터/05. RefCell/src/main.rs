/*
    RefCell<T>와 내부 가변성 패턴
        - 내부 가변성
            - 데이터에 대한 불변 참조자가 있을 때라도 데이터를 변형할 수 있게 해주는 러스트의 디자인 패턴
            - 원래는 빌림 규칙에 의해 허용되지 않지만 unsafe 코드를 사용
            - 단일 스레드에서만 사용 가능
            - 런타임에 수행됨
            - 같은 스코프에서 두개 이상의 가변참조자를 만들 수 없음
*/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let x = 5;
    // let y = &mut x; //error


    /*
        Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기
    */
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}


