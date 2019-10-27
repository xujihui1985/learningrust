#![allow(dead_code)]

pub fn if_statement() {
  let temp = 35;
  if temp > 30 {
    println!("it's hot");
  } else if temp < 10 {
    println!("cool");
  } else {
    println!("cold");
  }

  let day = if temp > 20 {"sunny"} else {"cloudy"}; // expression
  println!("today is {}", day);
}

pub fn while_loop() {

}