use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}",e),
            },
            other_error => panic!("Error opening the file: {:?}",other_error),
        },
    };

    //unwrap   match的快捷方法
    let f = File::open("hello.txt").unwrap();

    //expect
    let f = File::open("hello.txt").expect("无法打开");

}
