
fn main() {



  let result;
  {
    let s1 = String::from("a");
    let s2= String::from("b");
    result = lo(s1.as_str(), s2.as_str());
    println!("{}", result);
  }

}

fn lo<'a>(x: &str, y: &str) -> &'a str {
  x 
}