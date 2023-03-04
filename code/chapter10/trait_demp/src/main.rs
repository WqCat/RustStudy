use trait_demp::Summary;
use trait_demp::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("I hope miss you"),
        reply: false,
        retweet: false,

    };
    println!("1 new tweet: {}",tweet.summarize())
}