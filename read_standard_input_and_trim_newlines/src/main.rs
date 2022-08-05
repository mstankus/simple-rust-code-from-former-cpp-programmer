#![allow(unused)]
use std::io;

fn main() {
  let mut input = String::with_capacity(1000);
  loop {
    match io::stdin().read_line(&mut input) {
      Ok(n) => {
        if n==0 {
          break;
        }
        let slice = input.trim_end();
        println!("{} bytes: {}",slice.len(),slice);
      }
      Err(error) => println!("error: {error}")
    }
    input.clear();
  }
}
