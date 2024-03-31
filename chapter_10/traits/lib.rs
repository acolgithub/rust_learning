// newsarticle struct
pub struct NewsArticle {
    pub headline: String,  // headline of article
    pub location: String,  // location of article
    pub author: String,    // author of article
    pub content: String,   // article contents
}

// implementation of summary for newsarticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// tweet struct
pub struct Tweet {
    pub username: String,  // username of tweeter
    pub content: String,   // content of tweet
    pub reply: bool,       // determines if reply
    pub retweet: bool,     // determines if retweet
}

// implementation of summary for tweet struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize2(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// trait to get summary intended for article and tweet structs
pub trait Summary {
    fn summarize(&self) -> String;  // need to implement this trait in each struct having this trait
                                    // note that we could have included a default implementation which is overridden if we implement it in struct
    fn summarize2(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// return value implements summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as your probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}
