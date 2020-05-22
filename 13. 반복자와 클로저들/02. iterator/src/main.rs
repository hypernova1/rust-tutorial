/*
    반복자
*/

fn main() {
    //
    // 반복자 사용하기
    // iter 메소드는 불변 참조에 대한 반복자를 만듬
    // 가변 참조자 반복자를 만들고 싶다면 iter_mut 호출
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1 ,2, 3];

    // 인자로 넘긴 클로저는 호출되지 않음
    //     - 반복자를 소비하지 않기 때문
    v1.iter().map(|x| x + 1);

    // 반복자를 소비하기 위해 collect 함수를 사용
    // collect 함수는 다양한 컬렉션을 반환하기 때문에 자료형을 지정해주어야 함
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

/*
    모든 반복자는 표준 라이브러리에 정의된 Iterator 트레잇을 구현함

    trait Iterator {
        fn next(&mut self) -> Option<Self::Item>;
    }

    next 함수는 반복자를 소비함

*/


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); //sum 함수는 반복자의 소유권을 가지기 때문에 v1_iter는 더 이상 사용할 수 없음

    assert_eq!(total, 6);
}