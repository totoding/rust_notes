use std::fmt::Display;
struct Poit<T> {
  x: T,
  y: T
}

impl<T> Poit<T> {
    fn new(x:T, y:T) -> Poit<T>{
      Poit {
        x, y
      }
    }
}

impl Poit<i32> {
    fn area(&self) -> i32 {
      self.x * self.y
    }
}
fn main() {
  val(123);

  let p = Poit::new(1, 2);

  println!("{}", p.area());
  let p1 = Poit::new("x", "y");

  pri("123");
  print_where(p1);
}

fn pri<T: Display> (v: T) {
  println!("{}", v);
}

fn print_where<T> (v: T) 
  where T: Display
  {
    println!("{}", v);
  }
fn val<T>(v: T) -> T {
    v
}
