#[derive(Debug)] // Display를 하기 위한 어노테이션
struct User { // 구조체 정의
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {

    /*
        구조체
    */
    let user1 = build_user(String::from("hypemova@gmail.com"), String::from("sam"));

    let user2 = User { // 안스턴스 생성방법 1
        email: String::from("chtlstjd01@naver.com"),
        username: String::from("melchor"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user3 = User { // 인스턴스 생성방법 2
        ..user1
    };

    println!("{:?}", user3); // {:?}: debug 모드
    println!("{:#?}", user3); // {:#?} 포맷팅

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    /*
        튜플 구조체: 필드 타입만 정의할 수 있고 명명하지 않음
    */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
