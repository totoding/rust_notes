
pub fn text_learn() {
  let s1 = String::from("Hello");
  let s2 = String::from("World");
  let s3 = s1 + " " + &s2;
  println!("{}", s3);

  let s3 = String::from("Hello");
  let s4 = String::from("World");
  let s5 = format!("{} {}", s3, s4);
  println!("{}", s5);
  
  let s = "Hello World";
  let s6 = &s[0..1];
  println!("{}", s6);

  let s7 = s.to_string();
  println!("{}", s7);

  for c in s7.chars() {
    println!("{}", c);
  }

  for b in s7.bytes() {
    println!("{}", b);
  }
}