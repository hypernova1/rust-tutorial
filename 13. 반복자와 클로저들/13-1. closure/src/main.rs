/*
    클로저
        - 환경을 캡처할 수 있는 익명 함수

*/

use std::thread;
use std::time::Duration;

fn main() {

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout1(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout2(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout3(
        simulated_user_specified_value,
        simulated_random_number
    );

}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 운동 계획을 출력하는 함수
fn generate_workout1(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

/*
    리팩토링 (함수의 결과를 변수에 담음)
        - 모든 경우에 대해서 함수를 호출함
        - 결과를 사용하지 않는 블록에서도 비용 발생
*/
fn generate_workout2(intensity: u32, random_number: u32) {

    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} push ups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            )
        }
    }

    /*
        클로저 타입 추론과 어노테이션
    */
    fn add_one_v1(x: u32) -> u32 { x + 1 }                      //함수 정의
    let add_one_v2 = |x: u32| -> u32 { x + 1 };   // 파라미터, 반환 타입 모두 명시
    let add_one_v3 = |x| { x + 1 };                   // 타입 추론
    let add_one_v4 = x + 1;                                     // 표현식이 하나일 때 사용 중괄호 생략 가능


    // 서로 다른 타입 사용
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(2); //error! 첫 번째로 호출된 곳의 타입으로 타입이 정해짐

}

/*
    클로저를 사용한 리팩토링
        - 조건이 맞는 블록에서만 실행
*/
fn generate_workout3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} push ups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            )
        }
    }


}


