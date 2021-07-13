fn main() {
  let number: u8 = 4;

  match number {
    x if x == 0 => println!("Zero"),
    x if x > 0 => println!("Greater than zero"),
    _ => println!("Fell through"), // This should not be possible to reach
  }
}