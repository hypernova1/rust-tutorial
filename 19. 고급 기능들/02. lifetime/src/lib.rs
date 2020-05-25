/*
    서브타이핑 예제
        - Context내의 스트링 슬라이스와 Parser내의 Context를 가리키는 참조자에 대한 라이프타임 파라미터를 채워야함
*/
struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> { //'s: 'c: 's가 최소 'c만큼 오래 산다고 선언
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> { //Result의 Err 부분은 Parser 인스턴스 라이프타임에 묶여있음
        Err(&self.context.0[1..])
    }
}

fn parser_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse() //반환값의 라이프 타임이 Parser의 라이프타임에 묶여있음
}

/*
    제네릭 타입에 대한 참조자 상의 라이프타임 바운드
        - 제네릭 타입 내의 참조자들이 참조하고 있는 데이터보다 오래 살지 못하도록 제한
*/
struct Ref<'a, T: 'a>(&'a T);

// 'static 라이프타임 바운드를 T에 추가하여 T가 오직 'static 참조자만을 갖거나 아무런 참조자가 없도록 제한
struct StaticRef<T: 'static>(&'static T);

