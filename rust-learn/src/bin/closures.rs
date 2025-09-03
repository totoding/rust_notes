fn main() {
    let add_one = |x: i32| x + 1;

    let v = add_one(3);
    println!("{}", v);

    let mut c = 1;
    let mut add = || {
        c += 1;
    };
    add();
    add();
    add();
    println!("{c}");

    let fv = apply_fn(|x| x * 6);
    let fv1 = apply_fn2(|x| x * 6);
    println!("{}", fv);
    println!("{}", fv1)
}

fn apply_fn<T>(f: T) -> i32 where T: Fn(i32) -> i32 {
  f(10)
}

fn apply_fn2(f: impl Fn(i32) -> i32) -> i32 {
  f(20)
}