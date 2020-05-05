#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

/*
    메소드 정의하기
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    /*
    연관 함수
        - self 파라미터를 갖지 않는 함수 (메소드가 아닌 함수임 ex. String::from)
        - 주로 구조체의 인스턴스를 반환하는 생성자로 사용 됨
    */
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}


fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(3);
    println!("{:?}", square1);

}
