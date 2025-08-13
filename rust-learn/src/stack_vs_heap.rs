pub fn test() {
  stack_examples();
}

fn stack_examples() {
    // 基本类型存储在栈上
    let x: i32 = 42; // 整数
    let y: f64 = 3.14; // 浮点数
    let flag: bool = true; // 布尔值
    let ch: char = 'A'; // 字符

    // 固定大小的数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // 元组
    let tuple: (i32, f64) = (10, 20.5);

    // 结构体（如果所有字段都在栈上）
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 20 };
}

fn heap_examples() {
    // String - 动态字符串
    let s = String::from("Hello, World!");
    
    // Vec - 动态数组
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Box - 智能指针，将数据放在堆上
    let boxed_value = Box::new(42);
    
    // 大型结构体通常放在堆上
    let large_data = Box::new([0; 1000000]);
}