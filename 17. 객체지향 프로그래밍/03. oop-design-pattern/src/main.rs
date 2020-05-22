/*
    객체 지향 디자인 패턴 구현
        - 상태 패턴
            - 어떤 값이 상태 객체들의 집합으로 표현되는 내부상태를 가지고 이 값의 동작은 내부 상태에 기반하여 바뀜
*/

extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    let content = "I ate a salad for lunch today";
    post.add_text(content);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(content, post.content());
}
