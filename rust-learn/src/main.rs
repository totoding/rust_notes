mod user;
use crate::user::profile;
fn main() {
  profile::hi();
}

// // mod enum_learn;
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: i32
// }

// enum Width {
//     Int(i32),
//     Str(String),
// }
// struct Rect {
//     w: Width,
//     h: i32,
//     s: String
// }

// impl Rect {
//     fn get_area(&self) -> Result<i32, String> {
//         match &self.w {
//             Width::Int(val) => Ok(val * self.h),
//             Width::Str(_) => Err("abc".to_string())
//         }
//     }
//     fn from(w: Width, h: i32, s: String) -> Rect {
//         Rect { w, h, s }
//     }
// }
// // struct Color(i32, i32, i32);

// // struct any;
// fn main() {

//     let rec1 = Rect::from(Width::Int(23), 15, String::from("hahha"));
//     let area = rec1.get_area();
//     if let Ok(val) = area  {
//         println!("{}", val);
//     }
//     // let u1: User = User {
//     //     name: String::from("u1"),
//     //     age: 18
//     // };
//     // let u2 = User {
//     //     name: String::from("u2"),
//     //     ..u1
//     // };
//     // let u3 = User {
//     //     age: 17,
//     //     ..u1
//     // };
//     // println!("{}", u1.age);
//     // println!("{}", u2.age);
//     // println!("{}", u1.name); // ❌ u1 name 所有权被转移
//     // println!("{}", u3.name);

//     // let c1: Color = Color(255, 255, 255);
//     // println!("{}", c1.1)
//     // match enum_learn::test() {
//     //   Ok(v) => println!("{}", v),
//     //   Err(err) => println!("{}", err),
//     // }  ;
//     // let a: Option<i32> = Some(3);
//     // let b: Option<i32> = None;
//     // is_some(a);
//     // is_some(b);

//     // if let Ok(v) = enum_learn::test() {
//     //     println!("{}", v);
//     // } else {
//     //     println!("ccccc")
//     // }
// }

// // fn is_some(a: Option<i32>) {
// //     if let Some(v) = a { println!("{}", v) } else { println!("ccccc") }
// // }
