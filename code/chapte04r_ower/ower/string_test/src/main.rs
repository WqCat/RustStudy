fn main() {
    let mut s=String::from("Hello");
    s.push_str(",World");
    println!("the value of s:{}",s);
    println!("-------------------------");

    println!("Clone------heap");
    let s1 =String::from("Hello");
    let s2=s1.clone();
    println!("{},{}",s1,s2);

    println!("copy----stack");
    let x =5;
    let y =x;
    println!("{},{}",x,y);

    
}
