// Mark Stankus (c) Wed Aug  3 13:51:40 PDT 2022
use itertools::Itertools;

// The cartesian product doubles the number of characters
// to be processed.
//
// The map changes which will be gotten after dereferencing using iterator.

fn main() {
  let j = ("abcd".chars())
            .cartesian_product(0..2)
            .map(|x| -> char {
               if let 0=x.1%2 {
                 x.0.to_ascii_uppercase()
               } else { 
                 x.0
               }
             });
  for x in j {
    println!("{}",x);
  }
}
