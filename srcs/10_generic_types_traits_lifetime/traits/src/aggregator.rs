// trait declaration
pub trait Summary {
    fn summarize(&self) -> String;
    
    // default implementation
    fn summarize_default(&self) -> String {
        String::from("Read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}, {}, {}", self.headline, self.location, self.author, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
    
}

pub fn notify(item: &impl Summary) {
    println!("{} notified", item.summarize_default());
}
