mod enum_learn;
fn main() {
    // match enum_learn::test() {
    //   Ok(v) => println!("{}", v),
    //   Err(err) => println!("{}", err),
    // }  ;
    let a: Option<i32> = Some(3);
    let b: Option<i32> = None;
    is_some(a);
    is_some(b);

    if let Ok(v) = enum_learn::test() {
        println!("{}", v);
    } else {
        println!("ccccc")
    }
}

fn is_some(a: Option<i32>) {
    if let Some(v) = a { println!("{}", v) } else { println!("ccccc") }
}
