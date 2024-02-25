mod aggregator;

use aggregator::{NewsArticle, Tweet};

use crate::aggregator::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("acatsama"),
        content: String::from("I am a cat."),
        reply: String::from("ok"),
        retweet: String::from("12")
    };
    println!("{}", tweet.summarize());
    aggregator::notify(&tweet);
}
