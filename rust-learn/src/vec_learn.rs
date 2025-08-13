pub fn learn() {
    let arr = [1, 2, 3, 4, 5, 6];

    if let Some(val) = arr.get(2) {
        println!("{val}");
    }
    for element in arr {
        println!("值: {}", element);
    }
    for (_index, item) in arr.iter().enumerate() {
        println!("值: {}", item);
    }

    let mut arr1 = arr.to_vec();
    arr1.push(7);

    arr1.pop();
    dbg!(arr1);
}
