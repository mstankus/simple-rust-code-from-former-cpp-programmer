use std::{thread, time};
 
fn main () {
  let ten_millis = time::Duration::from_millis(10);
  println!(" I am going to keep for ten milliseconds."); 
  thread::sleep(ten_millis);
}
