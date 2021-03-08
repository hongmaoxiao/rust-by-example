// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An `enum` may either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  // or c-like structures.
  Click{x: i64, y: i64},
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    // Destructure `c` from inside the `enum`.
    WebEvent::KeyPress(c) => println!("key press '{}'.", c),
    WebEvent::Paste(s) => println!("pasted \"{}\".", s),
    // Destructure `Click` into `x` and `y`.
    WebEvent::Click {x, y} => println!("clicked at x={}, y={}.", x, y),
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click{x: 20, y: 80};
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  // We can refer to each variant via its alias, not its long and inconvenient
  // name.
  let x = Operations::Add;
  println!("Operations Add {:?}.", x);

  let add = Operations::Add.run(1, 2);
  let sub = Operations::Subtract.run(1, 2);
  println!("added {:?}.", add);
  println!("sub {:?}.", sub);
}