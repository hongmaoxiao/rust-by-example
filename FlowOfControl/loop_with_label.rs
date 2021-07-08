#![allow(unreachable_code)]

fn main() {
  'outer: loop {
    println!("Enter the outer loop");

    'inner: loop {
      println!("Enter the inner loop");

      // This would break only the inner loop
      //break;

      // This breaks the outer loop
      break 'outer;
    }

    println!("This point will never be reached");
  }

  println!("Exited the outer loop");
}