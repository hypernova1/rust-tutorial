/*
    Result와 함께하는 복구 가능한 에러

    Result 열거형은 Ok, Err 두개의 variant를 갖고 있음
*/

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f: u32 = File::open("hello.txt"); //error

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opning the file: {:?}", error);
    //     }
    // };

    /*
        서로 다른 에러에 대해 매칭하기
    */
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { //if error.kind() == ErrorKind::NotFound => 매치가드
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There wa a problem opening the file: {:?}", error)
        },
    };

    /*
        에러가 났을 때 패닉을 위한 숏컷: unwrap, expect
    */
    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");


}

/*
    에러 전파하기
        - 함수 내에서 에러를 처리하는 대신 에러를 호출하는 코드쪽으로 반환함
*/
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/*
    에러를 전파하기 위한 숏컷: ?
        - ?는 Result 타입을 반환하는 함수에서만 사용 가능
*/
fn read_username_from_file2() -> Result<String, io::ErrorKind> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}