use std::num::ParseIntError;

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  let first_number = match first_number_str.parse::<i32>() {
    Ok(first_number) => first_number,
    Err(e) => return Err(e),
  };

  let second_number = match second_number_str.parse::<i32>() {
    Ok(second_number) => second_number,
    Err(e) => return Err(e),
  };

  Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

fn main() {
  print(multiply("10", "2"));
  print(multiply("tt", "2"));
}