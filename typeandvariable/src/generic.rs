fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest;
}

fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

    for &item in list.iter() {
      if item > largest {
        largest = item;
      }
    }
    largest;
}

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self)
}
