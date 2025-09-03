trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}
impl Article {
    fn new(h: String, c: String) -> Article {
        Article {
            headline: h,
            content: c,
        }
    }
}
impl Summary for Article {
    fn summarize(&self) -> String {
        println!("summarize");
        self.content.clone()
    }
}
fn main() {

  let a = Article::new("h".to_string(), "c".to_string());
  let b = a.summarize();
  println!("{}", b);
  notify(&a)
}

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}