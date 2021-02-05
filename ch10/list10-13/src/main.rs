use list10_13::{self, Summary, Tweet, returns_summarizable};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("I new tweet: {}", tweet.summarize());

  let t = returns_summarizable();
  println!("@{}", t.summarize());
}