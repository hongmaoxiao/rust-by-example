use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
  v: Vec<i32>,
  u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
  v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(
  v: Vec<i32>,
  u: Vec<i32>,
) -> impl Iterator<Item = i32> {
  v.into_iter().chain(u.into_iter()).cycle()
}

// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
  let closure = move |x: i32| { x + y };
  closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
  numbers
      .iter()
      .filter(|x| x > &&0)
      .map(|x| x * 2)
}

fn main() {
  let v1 = vec![1, 2, 3];
  let v2 = vec![4, 5];

  let mut v3 = combine_vecs(v1, v2);
  assert_eq!(Some(1), v3.next());
  assert_eq!(Some(2), v3.next());
  assert_eq!(Some(3), v3.next());
  assert_eq!(Some(4), v3.next());
  assert_eq!(Some(5), v3.next());
  println!("all done");

  let plus_one = make_adder_function(1);
  assert_eq!(plus_one(2), 3);

  let singles = vec![-3, -2, 2, 3];
  let doubles = double_positives(&singles);
  assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}