fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
  // Let's try using `unwrap()` to get the number out. Will it bite us?
  let first_number = first_number_str.parse::<i32>().unwrap();
  let second_number = second_number_str.parse::<i32>().unwrap();

  first_number * second_number
}

fn main() {
  let twenty = multiply("10", "2");
  println!("double is {}", twenty);

  let tt = multiply("tt", "2");
  println!("double is {}", tt);
}