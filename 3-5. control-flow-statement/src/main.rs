fn main() {

    /*
       제어문
    */

    /*
        if 표현식
    */

    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("number is 5");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number2 = if condition {
        5
    } else {                            // let 구문에서 if 사용
        6
    };

    println!("The value of number2 is: {}", number2);



    /*
        반복문
    */

    /*
        loop
    */
    loop {
        print!("again");
        if condition {
            break;
        }
    }

    /*
        while
    */
    let mut number3 = 3;

    while number3 != 0 {
        println!("The value of number3 is: {}", number3);
        number3 = number3 - 1;
    }

    /*
        for
    */
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the values of arr are: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
