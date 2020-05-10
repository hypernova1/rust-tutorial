use std::thread;
use std::time::Duration;

/*
    제네릭 파라미터와 Fn 트레잇을 사용하여 클로저 저장하
*/
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

// Cacher는 처음 호출했을 때의 결과값을 계속 반환하기 때문에 수정이 불가능
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 2); //error
}

/*
    클로저로 환경 캡처
        클로저의 3가지 캡처 방식
            1. FnOnce
                - 소유권 빌려오기
                - 클로저의 환경을 알고 있는, 그것을 둘러싼 환경에서 캡처한 변수들을 소비
                - 캡처한 변수를 소비하기 위해 클로저는 이 변수의 소유권을 가져야하고 그것이 정의될 때 클로저 안으로 옮겨와야 함
                - 한 번만 호출 가능
            2. Fn
                - 불변으로 빌려오기
                - 환경으로 부터 값들을 불변으로 빌려옴
            3. FnMut
                - 가변으로 빌려오기
                - 환경으로 부터 값들을 가변으로 불러옴
*/
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    //move 클로저
    let equal_to_x = move |z| z == x; //클로저가 정의 될 때 x의 값이 클로저 안으로 이동됨, x의 소유권은 클로저가 가짐

    // println!("can't use x here: {:?}", x); //error

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

}
