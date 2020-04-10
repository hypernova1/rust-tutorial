/*
제네릭 데이터 타입
*/

/*
    이름과 시그니처만 다른 두 함수들
        - 타입만 다르고 내부 구현은 똑같다.
*/
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
    제네릭 사용
*/

// 하나의 타입
struct Point<T> {
    x: T,
    y: T,
}

// 열거형 정의 내에서 제네릭 타입 사용
enum Option<T> {
    Some(T),
    None,
}

// 열거형 정의 내에서 여러개의 제네릭 타입 사용
enum Result<T, E> {
    Ok(T),
    Error(E),
}

// 메소드 정의 내에서 제네릭 데이터 타입 사용
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 여러개의 타입
struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


// fn largest<T>(list: &[T]) -> T {
//
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if (item > largest) {
//             largest = item;
//         }
//     }
//
//     largest
// }


fn main() {

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.1 };
    // let wont_work = Point { x: 1, y: 1.5 }; //Error T만 가지고 있기 때문에 동일한 타입어야함

    let integer_and_float = Point2 { x: 1, y: 1.5 }; //T, U를 가지고 있기 때문에 다른 타입 사용 가능


    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}