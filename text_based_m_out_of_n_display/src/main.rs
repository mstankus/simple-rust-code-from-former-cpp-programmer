use std::{thread, time}; // to slow down the loop
use indicatif::HumanCount;

fn main() {
  let ten_millis = time::Duration::from_millis(500);
  let mut len = 0;
  let mut line = String::with_capacity(1000);
  let max = 20;
  for i in 0..=max {
    line.clear();
    for _ in 0..len {
      line += "\x08";
    }
    let line = format!("{}Counter : {} of {}",
                       line,
                       HumanCount(i),
                       max);
    eprint!("{}",line);
    len = line.len();
    thread::sleep(ten_millis);
  }
  println!();
}
