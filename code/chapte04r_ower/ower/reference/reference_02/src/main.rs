fn main() {
    //防止引用时的数据竞争
    println!("①.不在同一作用域");
    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
        println!("{}",s1);
    }
    let s2 = &mut s;
    //let s3 = &mut s;
    println!("{}",s2);
    println!("②.1.不可以有一个可变的引用和一个不可变的引用 \n' '2.可以有多个不可变的引用");
    println!("③:悬空引用");
}
