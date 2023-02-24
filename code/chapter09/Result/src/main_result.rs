use std::error::Error;
use std::fs::File;

fn main() -> Result<(),Box<dyb Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}