/*
    제네릭, 트레잇, 라이프타임

*/

fn main() {

    /*
        중복되는 코드가 많다.
    */
    // let numbers = vec![34, 50, 25, 100, 65];

    // let mut largest = numbers[0];

    // for number in numbers {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let mut largest = numbers[0];

    // for number in numbers {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);


    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

}

/*
    함수를 추출하여 중복 없애기
*/
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}