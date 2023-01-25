#![allow(dead_code, unused_variables)]

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct InstagramPost {
    username: String,
    caption: String,
}

impl Summary for InstagramPost {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let instagrampost = InstagramPost {
        username: String::from("monkey_ebooks"),
        caption: String::from("horse_ebooks"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new Instagram post: {}", instagrampost.summarize());
}
