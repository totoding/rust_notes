fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let l = get_large(&arr);
    println!("{}", l);
    dbg!(arr);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = get_large(&chars);
    println!("最大的字符是 {}", result);
}

fn get_large<T>(arr: &[T]) -> &T 
  where T: PartialOrd + Copy
{
    let mut lar = &arr[0];
    for item in arr {
        if item > lar {
            lar = item;
        }
    }
    lar
}
