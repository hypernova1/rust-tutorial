/*
    Drop
        - 메모리 정리 코드 실행
        - 값이 스코프 밖으로 벗어나려고 할 때의 상황을 커스터마이징할 수 있음
*/

use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("hello") };
    let d = CustomSmartPointer { data: String::from("world") };

    // c.drop(); //drop을 명시적으로 호출하는 것은 허용되지 않음. 동일한 값을 두 번 해제할 수도 있기 때문(중복해제 방지)
    // 강제하길 원한다면 std::mem:drop 함수 사용
    drop(c);


    println!("CustomSmartPointers created.");
} // 역순으로 drop이 호출된다.
