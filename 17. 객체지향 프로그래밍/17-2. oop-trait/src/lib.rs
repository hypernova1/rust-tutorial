/*
    공통된 동작을 위한 트레잇 정의
*/

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //dyn: 트레잇 오브젝트와 구조체를 구분하기 위한 키워드
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
    트레잇 구현하기
*/
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button!");
    }
}