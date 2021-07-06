fn main() {
  let mut _mutable_integer = 7i32;

  println!("before mutable: {}", _mutable_integer);

  {
    // Shadowing by immutable `_mutable_integer`
    let _mutable_integer = _mutable_integer;

    // Error! `_mutable_integer` is frozen in this scope
    // _mutable_integer = 50;
    // FIXME ^ Comment out this line

    // `_mutable_integer` goes out of scope
  }

  _mutable_integer = 3;
  println!("after mutable: {}", _mutable_integer);
}