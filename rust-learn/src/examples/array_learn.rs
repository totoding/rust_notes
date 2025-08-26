pub fn lear() {
  let arr = [1, 2, 3, 4, 5];

  for (index, item) in arr.iter().enumerate()  {
      println!("{},{}", index, item);
  }

  let new_arr = &arr[0..2];
  let new_arr1 = &arr[1..8];
  println!("{:?}", new_arr);
  println!("{:?}", new_arr1);
}