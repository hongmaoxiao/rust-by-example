fn foo() -> ! {
  panic!("This call never returns.");
}

fn some_fn() {
  ()
}

fn main() {
  let _a: () = some_fn();
  println!("This function returns and you can see this line.");
}