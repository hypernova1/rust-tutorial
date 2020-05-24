/*
    안전하지 않은 러스트
        - 로우 포인터를 역참조
        - 안전하지 않은 함수 혹은 메소드 호출
        - 가변 정적 변수의 접근 혹은 수정
        - 안전하지 않은 트레잇 구현

        - unsafe 블록 안에서 실행되어야 함
        - unsafe는 빌림 검사기나 다른 러스트의 안전성 검사 기능을 끄는 것이 아님
        - 위의 네 가지 경우가 아니라면 컴파일러는 발생시킴
        - 안전하지 않은 코드를 안전한 추상화 내에 있도록 감싸서 안전한 API를 제공하는 것이 최상
*/

/*
    로우 포인터 역참조
        - 로우 포인터: *const T, *mut T
            - *은 역참조 연산자가 아닌 타입 이름의 일부
        - 로우 포인터의 성질
            - 빌림 규칙 무시가 허용되어 불변 및 가변 포인터 양쪽 모두를 갖거나 같은 위치에 여러개의 가변 포인터를 가질 수 있음
            - 유효한 메모리를 가리키는 것을 보장하지 않음
            - null이 될 수 있음
            - 자동 메모리 정리가 구현되어 있지 않음
*/

use std::slice;
use std::convert::TryInto;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {

    let mut num = 5;

    // 불변 및 가변 로우 포인터 만들기
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}, r2 is: {}", *r1, *r2); //역참조 연산자를 사용하여 값을 읽음

        dangerous(); //안전하지 않은 함수 호출
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    /*
        slice::from_raw_parts_mut 사용은 슬라이스에 사용될 때 크래시를 일으키기 쉬움
    */
    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000) //임의의 메모리 취치를 얻어서 10000개의 아이템 길이를 갖는 슬라이스 생성
    };
    /* 
        address에 저장된 임의의 메모리 위치는 소유된 값이 아니어서 슬라이스가 유호한 i32값들을 담고있음을 보장하지 않음
        slice를 유효한 슬라이스인 것처럼 사용하는 시도는 정의되지 않은 동작(undefined behaviour)을 야기함
    */


     //러스트는 C의 함수가 유효한지 검사할 수 없기 때문에 unsafe 블록 안에서 호출
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    /*
        가변 정적 변수 (가변 전역 변수)
            - 상수와 비슷하지만 정적 변수는 값이 메모리 내에 고정된 주소값을 가짐
                - 상수는 사용될 때마다 데이터가 복사되는 것이 허용
            - 정적 변수는 가변일 수 있음
    */
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER); //다중 스레드에서 가변 정적 변수는 데이터 레이스를 일으키기 쉬움
    }
}

/*
    extern
        - 외국(다른 프로그래밍 언어) 함수 인터페이스의 생성과 사용을 가능하게 함
*/
// C언어 함수 호출
extern "C" {
    fn abs(input: i32) -> i32;
}

// 다른 언어로부터 러스트 함수 호출
#[no_mangle] //러스트 컴파일러가 이 함수의 이름을 맹글링하지 않도록 함 *맹글링: 함수의 이름을 컴파일 과정에서 (사람들이 읽기에 안좋은) 다른 이름으로 바꾸는 과정
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


// 안전하지 않은 함수 생성
unsafe fn dangerous() {}

// 안전하지 않은 코드상에 안전한 추상화 생성
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //로우 포인터 접근 => *mut i32

    assert!(mid <= len); //mid가 len보다 반드시 작거나 같다는 단언 추가

    // (&mut slice[..mid], &mut slice[mid..]) //슬라이스의 서로 다른 부분을 빌리기 때문에 에러, 같은 슬라이스를 두 번 빌림

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset((mid as usize).try_into().unwrap()), len - mid))
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/*
    안전하지 않은 트레잇 구현
        - Send되지 않거나 Sync하지 않은 타입을 포함한 타입을 구현하고, Send되거나 Sync한 것으로 표시하려면 unsafe를 이용
*/
unsafe trait Foo {}

unsafe impl Foo for i32 {}