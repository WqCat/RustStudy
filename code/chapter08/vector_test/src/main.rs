fn main() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];  //读取vector的元素
    println!("The third element is {}",third);
    
    match v.get(2) {   //读取vector的元素
        Some(third) => println!("The third element is {}",third),
        None => println!("There is no third element"),
    }

    let mut m = vec![1,2,3,4,5];
    let first = &v[0];   //不可变的借用
    //  v.push(6);      //此为可变的,所以这是报错
    println!("The first element is {}",first);   //不可变的借用

    //vector遍历
    let mut n = vec![100,50,60];
    for i in &mut n {
        *i +=50;
    }

    for i in n {
        println!("{}",i);
    }


}
