use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // panic
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was an error: {:?}", error)
    //     }
    // };

    // let f = File::open("hello.txt").map_err(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:#?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:#?}", error);
    //     }
    // });

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open file");
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    // ? 运算符将错误返回给调用函数
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}