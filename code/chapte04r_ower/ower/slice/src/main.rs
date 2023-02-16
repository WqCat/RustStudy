fn main() {
    let s1 = String::from("Hello World");

    let hello = &s1[..5];   //[0..5]
    let world = &s1[6..];   //[6..11]

    let whole = &s1[..];    //[0..s.len()]
    println!("{},{},{}",hello,world,whole);

    //找空格的例子
    println!("找空格的例子");

    let mut s = String::from("Hello World");
    let worldIndex = first_world(&s);  //此时s为不可变的引用

    //s.clear();  // s已经被定义为不可变的，不能清理  
    println!("{}",worldIndex);

}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
