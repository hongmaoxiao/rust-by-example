fn main() {
  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  // Try changing the values in the struct to see what happens
  let foo = Foo {
    x: (3, 2),
    y: 3
  };

  match foo {
    Foo { x: (1, b), y } => println!("First of x is 1, b is {}, y is {}", b, y),

    // you can destructure structs and rename the variables,
    // the order is not important
    Foo { y: 2, x: i } => println!("y is 2, and x is {:?}", i),

    // and you can also ignore some variables:
    Foo { y, .. } => println!("y is {}, we don't care about others!", y),
    // this will give an error: pattern does not mention field `x`
    // Foo { y } => println!("y = {}", y),
  }
}