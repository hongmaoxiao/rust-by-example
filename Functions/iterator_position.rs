fn main() {
  let vec = vec![1, 9, 3, 3, 13, 2];

  let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
  assert_eq!(index_of_first_even_number, Some(5));

  let index_of_first_negative_number = vec.iter().position(|x| x < &0);
  assert_eq!(index_of_first_negative_number, None);
}
