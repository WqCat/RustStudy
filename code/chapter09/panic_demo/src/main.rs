pub struct Guess {
    value: i32,
}


impl Guess {
    pub fn new(value: i32) -> Guess {  //关联函数new
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100!,got {}",value);
        }

        Guess{value}
    }

    pub fn value(&self) -> i32 {  //方法 value
        self.value
    }

}

fn main() {
    loop {
        let guess = "32";
        let guess: i32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

    }
}
