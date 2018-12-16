trait Summary {
    fn summarize(&self)->String {
        String::from("(Read more from {}...)")
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

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Compionship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    notify(tweet);
    //println!("1 new tweet: {}", tweet.summarize());
    println!("New article availabel! {}", article.summarize());
}
