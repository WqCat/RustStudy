pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {   //类似有初始化，有具体实现
        format!("Read more from:{}",self.summarize_author())   //summarize_author can be called
    }
}

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
