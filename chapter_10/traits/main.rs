// bring traits and struct into scope
use traits::{Summary, Tweet, NewsArticle};

fn main() {
    
    // create tweet
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Leafs win the Stanley Cup Championship!"),
        location: String::from("Toronto, ON, CAN"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Toronto Maple Leafs once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize2());
}
