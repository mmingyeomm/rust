use std::fmt::format;


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String, 
    pub content: String,
    pub reply:bool,
    pub retweet:bool,
}


pub trait Summary{
    fn summarize(&self) -> String;
}


impl Summary for NewsArticle{
    fn summarize(&self) -> String {

        format!("{}: {}", self.headline, self.content)

    }
}

impl Summary for Tweet{
    fn summarize(&self) -> String {

        format!("{}: {}", self.username, self.content)

    }

}




fn main() {

    let tweet:Tweet = Tweet{
        username: String::from("mingyeom"),
        content: String::from("hello guys"),
        reply: bool::from(true),
        retweet: bool::from(true),
    };


    
    println!("Tweet Summary : {} ", tweet.summarize())
}
