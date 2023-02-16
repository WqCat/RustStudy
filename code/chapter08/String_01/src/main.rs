fn main() {
    //String初始化
    let v = "asfafs";       //&str类型
    let m = v.to_string();  //String类型
    let n = String::from("s: &str"); //String

    //更新String
    let mut a = String::from("s: &str");
    let s1 = String::from("45210");
    a.push_str(&s1);  //push_str你会获得参数的所有权
    a.push('s');    //附加单个字符
    println!("{},{}",a,s1);

    //String的拼接
    let a1 = String::from("luan");
    let a2 = String::from("kong");
    let a3 = String::from("100");

    // let a4 = a1 + "-" + &a2 + "-" + &a3;   //此+用的 add(self,s: &str){} ，第一个数会抢所有权
    // println!("{},{},{},{}",a1,a2,a3,a4);   //a1所有权被抢
    
    let a5 = format!("{}-{}-{}", a1, a2, a3); //不会抢所有权

    println!("{}!",a5);
}
