fn main() {
    let string1 = String::from("asdsa");
    let string2 = "asd";
    let result = longest(string1.as_str(), string2); 
    println!("The longest is {}", result);
}

fn longest<'a>(x:&'a str,y: &str) -> String {
    let result = String::from("abas");
    result
}

// fn longest<'a>(x:&'a str,y: &str) -> &'a str {    //这个是引用，到最后的result.as_str()被清理了
//     let result = String::from("abas");
//     result.as_str()
// }

//生命周期的省略：1.每个引用类型都有自己的生命周期哦
//              2.如果只有一个输入的生命周期参数，这个生命周期将付给所有的输出周期
//              3.多个输入的生命周期参数，有一个&self或&mut self ，那么self生命周期将赋值给所有的输出生命周期
