fn main() {
    another_function(1, 3.2);

    let x = 3;
    let y = {
        let x = 1;  // 표현식
        x + 5
    };

    println!("====================");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let z = five();
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {  // 반환 값을 갖는 함수
    5
}
