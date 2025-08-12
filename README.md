### day1 为什么学rust
跟风 随便看看 
### day2 变量 基本类型

#### 变量
```
  let a = 1; // 变量默认不可变
  a = 2; // ❌ cannot mutate immutable variable `a`
  let a = 2; // ✔ 变量遮蔽

  let str = " ";
  let str = str.len();  // ✔ 变量遮蔽可改变类型

  let mut b: i32 = 3 // 使用 mut 后 变量b 可变
  b = 4; // ✔ 
  b = "5" // ❌ 类型不可变
```
#### 整数类型

##### 有符号整数 (Signed Integers)
- `i8`:   8位，范围: -128 到 127
- `i16`:  16位，范围: -32,768 到 32,767
- `i32`:  32位，范围: -2,147,483,648 到 2,147,483,647 (默认整数类型)
- `i64`:  64位，范围: -9,223,372,036,854,775,808 到 9,223,372,036,854,775,807
- `i128`: 128位，范围: -(2^127) 到 2^127-1
- `isize`: 架构相关大小 (32位系统上为32位，64位系统上为64位)

##### 无符号整数 (Unsigned Integers)
- `u8`:   8位，范围: 0 到 255
- `u16`:  16位，范围: 0 到 65,535
- `u32`:  32位，范围: 0 到 4,294,967,295
- `u64`:  64位，范围: 0 到 18,446,744,073,709,551,615
- `u128`: 128位，范围: 0 到 2^128-1
- `usize`: 架构相关大小，通常用于索引

##### 数字字面量表示法
```rust
let decimal = 98_222;        // 十进制，可以用下划线分隔
let hex = 0xff;              // 十六进制
let octal = 0o77;            // 八进制
let binary = 0b1111_0000;    // 二进制
let byte = b'A';             // 字节字面量 (仅限 u8)
```


#### 浮点数类型  f32 f64  
##### 特殊值  
```rust
let infinity = f64::INFINITY;
let neg_infinity = f64::NEG_INFINITY;
let nan =f64::NAN;
```
#### 布尔类型 bool
bool true | false  

#### 字符类型 char  
`char` 类型表示单个 Unicode 标量值 大小4字节 单引号包围 支持所有Unicode字符  

#### 字符串  
##### 字符串字面量 &str  
- 不可变引用  
- 储存在程序的二进制文件里  
- 编译时已知大小  
```rust
let s = "Hello World";
```
##### String 类型  
- 可变,可增长的字符串  
- 储存在堆上  
- URF-8 编码  
```rust
let mut s = String::new();
s.push_str("Hello");
s.push(' ');
s.push_str("World");
```