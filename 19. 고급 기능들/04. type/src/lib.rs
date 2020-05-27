use std::io::Error;
use std::fmt;
use std::result::Result;
use std::io::Result as Result2;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<usize, Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}


/*
    별칭을 사용하여 Result<T, E> 타입 반복 줄이기
    Result<T, E> 는 반복이 많기 때문에 std::io에 아래와 같이 별칭 선언이 되어있음
        - type Result<T> = Result<T, std::io::Error>;
*/
pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<usize>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<usize>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}


