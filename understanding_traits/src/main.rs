pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // only method signature but body
    fn summarize(&self) -> String {
        String::from("(Read more)..")
    }

    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
//     //...
// }

// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
//     //..
// }

// fn some_function<T, U>(t: &T, u: &U) -> i32 
//         where T: Display + Clone, 
//                 U: Clone + Debug {

//                 }

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling."),
    };

    notify(&article);

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
