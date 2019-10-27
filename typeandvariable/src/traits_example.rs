pub trait Summary {
  fn summarizehead(&self) -> String;

  fn summarize(&self) -> String {
    format!("hello {}", self.summarizehead())
  }
}

pub struct NewsAricle {
  pub headline: String,
  pub location: String,
  pub author: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarizehead(&self) -> String {
    format!("{}, by {} ({})", self.content, self.username,self.reply)
  }
}

impl Summary for NewsAricle {
  fn summarizehead(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author,self.location)
  }
}

// Trait Bounds
pub fn notify<T: Summary>(item: T) {
  println!("Breaking news! {}", item.summarize())
}

fn some_function<T, U>(t: T, u: U) -> i32 
  where T: std::Display + Clone,
        U: Clone + Debug
{

}