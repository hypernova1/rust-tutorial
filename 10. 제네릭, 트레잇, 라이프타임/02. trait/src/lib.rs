use std::fmt::{Display, Debug};

/*
    트레잇
        - 다른 종류의 추상화를 사용할 수 있게 해줌
        - 타입들이 공통적으로 갖는 동작에 대하여 추상화
        - 다른 언어에서 사용되는 인터페이스와 유사하지만 몇 가지 다른점이 있음
*/

// 트레잇 정의하기
pub trait Summarizable {
    fn summary(&self) -> String;
}

// 특정 타입에 대한 트레잇 구현하기
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// for 뒤에 트레잇을 구현하고자 하는 타입의 이름을 씀
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summarizable for Tweet {
//     fn summary(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


// 기본 구현
pub trait Summarizable2 {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summarizable2 for NewsArticle {}


// 기본 구현을 갖지 않은 메소드 사용
pub trait Summarizable3 {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...", self.author_summary())
    }
}

// 구현
impl Summarizable3 for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}


/*
    트레잇 바운드
        - 제네릭을 특정한 타입들로 제한
*/
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

// 여러개의 트레잇 바운드 특정하기
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    0
}

// where절을 사용하여 가독성 높이기
fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}
