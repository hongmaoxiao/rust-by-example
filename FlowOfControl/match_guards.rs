fn main() {
  let pair = (2, -2);
  // TODO ^ Try different values for `pair`

  println!("Tell me about {:?}", pair);

  match pair {
    (x, y) if x == y => println!("These are twins"),
    // The ^ `if condition` part is a guard
    (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
    (x, _) if x % 2 == 0 => println!("The first one is odd"),
    _ => println!("No correlation..."),
  }
}