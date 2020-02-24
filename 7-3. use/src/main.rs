/*
    use: 이름 가져오기
*/

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;

use TrafficLight::{Red, Yellow}; // 여러개 가져오기

// use TrafficLight::*; // 모두 가져오기  // *: grob

fn main() {
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}