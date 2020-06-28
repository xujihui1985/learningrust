mod temporary_variable;
mod split_struct;
use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 3];
    let list_first = list.first();
    let list_last = list.last();
    println!("first {:?} last {:?}", list_first, list_last);

    let first_mut = list.first_mut().expect("list was empty");
    *first_mut = 3;

    temp_variabels();
    counting_words();
    counting_words_with_entry_api();
    monster();
}

// new scopes
// tells rust where borrows end
fn add_new_scope() {
    let mut list = vec![1, 2, 3];
    {
      let list_first = list.first();
      let list_last = list.last();
      println!("first {:?} last {:?}", list_first, list_last);
    }

    let first_mut = list.first_mut().expect("list was empty");
    *first_mut = 3;
}

// temporary variables
fn temp_variabels() {
  let mut player1 = temporary_variable::Player::new();
  player1.set_score(player1.score() + 1);
  println!("score {:?}", player1.score());
}

fn counting_words() {
  let text = "hello world hello";
  let mut freqs = HashMap::<&str, i32>::new();
  for word in text.split_whitespace() {
    match freqs.get_mut(word) {
      Some(value) => *value += 1,
      None => {
        freqs.insert(word, 1);
      }
    }
  }

  println!("word frequences: {:#?}", freqs);
}

fn counting_words_with_entry_api() {
  let text = "hello world hello";
  let mut freqs = HashMap::<&str, i32>::new();
  for word in text.split_whitespace() {
    *freqs.entry(word).or_insert(0) += 1;
  }

  println!("word frequences: {:#?}", freqs);
}

fn monster() {
  let mut m = split_struct::Monster::new();
  m.final_breath();
}