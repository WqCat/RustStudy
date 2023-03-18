fn main() {
    let s =String::from("Hello World");
    take_owership(s);  //进入函数直接没用了
   // println!("{}",s);//进入函数直接没用了

    let x = 5;
    make_copy(x);
    println!("x:{}",x);
}

fn take_owership(some_string: String){
    println!("{}",some_string);
}

fn make_copy(some_string: i32){
    println!("{}",some_string);
}