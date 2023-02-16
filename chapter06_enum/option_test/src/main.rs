fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v= 0u8;  //256 种
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  //以下的省略，_通配符
    }

    let s = Some(0u8);
    //以下 if let 用法,只关心一种
    if let Some(3) = s {
        println!("three");
    } else{
        println!("others");
    }
}

fn plus_one(x: Option<i32>) ->Option<i32> {
    match x {  //以下两个都要写
        None => None,
        Some(i) => Some(i+1),
    }
}