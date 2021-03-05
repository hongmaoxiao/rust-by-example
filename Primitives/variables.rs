fn main() {
  // Variables can be type annotated.
  let logical: bool = true;
  println!("logical: {}", logical);

  let a_float: f64 = 1.0; // Regular annotation
  let an_integer = 5i32; // Suffix annotation
  println!("a_float: {:.3}", a_float);
  println!("an_integer: {}", an_integer);

  // Or a default will be used.
  let default_float = 3.0; // `f64`
  let default_integer = 7; // `i32`
  println!("default_float: {:.5}", default_float);
  println!("default_integer: {}", default_integer);

  // A type can also be inferred from context
  let mut inferred_type = 12; // Type i64 is inferred from another line
  inferred_type = 4294967296i64;
  println!("inferred_type: {}", inferred_type);

  // A mutable variable's value can be changed.
  let mut mutable = 12; // Mutable `i32`
  mutable = 21;
  println!("mutable: {}", mutable);

  // Error! The type of a variable can't be changed.
  // mutable = true;
  // println!("mutable: {}", mutable);

  // Variables can be overwritten with shadowing.
  let mutable = true;
  println!("overwritten mutable: {}", mutable);
}