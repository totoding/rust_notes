
pub fn tup_learn() {
  let tup = (1, 2, 4, 5);

  println!("{:?}", tup);
  println!("{}", tup.2);

  let (x, y, _, _) = tup;
  println!("{}, {}", x, y);

}