pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {   //类似有初始化，有具体实现
        format!("Read more from:{}",self.summarize_author())   //summarize_author can be called
    }
}

use std::fmt::Display;
//新增
pub fn notify01(item: impl Summary + Display) {   //impl Summary 表明实现了Summary的某个函数，    有其他类型是实现了Summary，那个类型可以使用该方法
    println!("ASFLKN:{}",item.summarize())
}

pub fn notify02<T: Summary + Display>(item1: T,item2: T) {   //同上，更简单
    println!("ASFLKN:{}",item1.summarize())
}

use std::fmt::Debug;
pub fn notify03<T,U>(a: T,b: U) -> String   //where,看起来更好看
where 
    T: Summary +Display,
    U: Clone +Debug,
{   
    println!("ASFLKN:{}",a.summarize())
}

//实现trait作为返回类型 ,其返回的类型必须是同一个，不能返回两种可能的类型,例如：if/else两种情况
pub fn notify04(s: &str) -> impl Summary {   //同上，更简单
    NewArticle {
        headline: String::from("aefafasf"),
        location: String::from("aefafasf"),
        author:String::from("aefafasf"),
        content: String::from("aefafasf")
    }
}

//最后一个，让任何实现了Display的类型调用ToString
//  impl<T: fmt::Display> ToString for T {}







pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {   
    fn summarize_author(&self) -> String {   //只有
        format!("@{}",self.author)
    }

    fn summarize(&self) -> String {          //because summarize_author be called,so summarize_author must be initialize 
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize_author(&self) -> String{
    //     format!("@{}",self.String)
    // }

    fn summarize_author(&self) -> String {   //只有
        format!("@{}",self.username)
    }

    fn summarize(&self) -> String {
        format!("{}:{}",self.username,self.content)
    }
}
