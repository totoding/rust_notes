
// #[derive(Debug)]
// enum Direction {
//     North,
//     South,
//     West,
//     East,
// }

// type Poit = Vec<Direction>;

// enum User {
//     Name(String),
//     Age(i32),
//     Local {
//         x: f32,
//         y: f32,
//     },
//     Val(Poit),
// }

// impl Direction {
//     fn all() -> Vec<Direction> {
//         vec![Direction::North, Direction::South, Direction::West, Direction::East]
//     }
// }
use rand::prelude::*;

pub fn test() -> Result<i32, String> {
    let mut rng = rand::rng(); 
    let n: i32 = rng.random_range(0..10); 

    let v = if n > 5 {
        Some(5)
    } else {
        None
    };
    let y: Option<i32> = None;
    let v1 = y.unwrap_or(3);

    println!("{}", v1);
    match v {
        Some(val) => Ok(val),
        None => Err(String::from("None")),
    }
}
