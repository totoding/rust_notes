
use std::collections::HashMap;

#[derive(Debug)]
struct Person {
  name: String,
  age: i32
}

pub fn test() {
    let s = String::from("a");

    let s1 = &s;

    assert_eq!("a", *s1);
    // let mut hm = HashMap::new();

    // let mut key: String = String::from("key");
    // let val = 3;
    // hm.insert(key, val);
    // dbg!(&hm);
    // key = "key1".to_string();
    // println!("{}", key);
    // dbg!(hm);
    // let s1 = String::from("gh");
    // println!("s1 地址: {:p}", s1.as_ptr());
    // let s2 = s1;
    // println!("s2 地址: {:p}", s2.as_ptr());
    // // println!("{},{}", s1, s2);
    // takes_ownership(s2);

    // let s3 = String::from("a");
    // println!("s1 地址: {:p}", s3.as_ptr());

    // let u1 = Person {
    //   name: s3,
    //   age: 18
    // };

    // println!("{:?}", u1);
    
    // let u2 = Person {
    //   name: u1.name,
    //   age: 5
    // };
    
    // println!(" {}, {}, {}", u1.age, u2.name, u2.age);

    // let mut vec = vec![
    //   String::from("a"),
    //   String::from("a"),
    //   String::from("a"),
    // ];

    // vec.remove(0);
    // println!("{:?}", vec);

    // for item in  vec.into_iter(){
        
    // }

    // for item in  vec {
        
    // }

    // for item in vec.iter() {

    // }

    // for item in vec.iter().enumerate() {

    // }

}

fn takes_ownership(s: String) {
    println!("函数内: {}", s);
} // s 在这里被 drop
