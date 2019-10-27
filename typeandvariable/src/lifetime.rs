fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn make_important_excerpt() {
  let novel = String::from("hello world.aaabbb");
  let first_sentence = novel.split('.').next().expect("fadsff");
  let i = ImportantExcerpt {
    part: first_sentence
  };
}