use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"),String::from("Yellow")];  //vector类型
    let intial_scores = vec![10,50];

    let scores: HashMap<_,_> =
        teams.iter().zip(intial_scores.iter()).collect();  //zip 类似于拉链的效果
                                                           // collect把两个转化为HashMap类型
    
    //插入时如果不加&，所有权就没了
     let mut peoples = HashMap::new();
    let name = String::from("2ban");
    let number = 2;
    peoples.insert(&name,&number);
    println!("{},{}",name,number);
    
    //get方法
    let search_name = String::from("name");
    let result = peoples.get(&search_name);  //result is option type,use math to slove it 

    match result {
        None => println!("no exist!"),
        Some(s) => println!("{}",s),
    }

    //for循环
    let name = String::from("1ban");
    let number = 1;
    peoples.insert(&name,&number);

    for (k,v) in &scores {
        println!("{}: {}",k,v);
    }


    //update method
    let mut color = HashMap::new();
    
    //覆盖
    color.insert(String::from("Blue"),20);
    color.insert(String::from("Blue"),60);
    println!("被覆盖的结果{:?}",color);

    //entry
    color.entry(String::from("Blue")).or_insert(50);   //存在，不改变
    color.entry(String::from("Yellow")).or_insert(10);  //不存在，加入
    println!("entry:{:?}",color);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    } 
    println!("{:#?}",map);




}
