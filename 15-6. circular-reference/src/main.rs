/*
    순환참조
*/

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, RefCell<Rc<List>>),
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {

    /*
        순환참조 만들기
    */
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 순환참조 에러
    // println!("a next item {:?}", a.tail());

    println!("===========================");

    /*
        순환참조 방지하기: Rc<T>를 Weak<T>
            - 약한 참조
                - 강한참조 개수가 0이되면 약한 참조도 같이 깨짐
    */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}",
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch));

        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("leaf string = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));
}

// 부모는 소유하지 않고 자식만 소유함
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}