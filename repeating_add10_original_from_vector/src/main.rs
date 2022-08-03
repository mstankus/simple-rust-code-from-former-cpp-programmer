// Mark Stankus (c) Wed Aug  3 13:58:59 PDT 2022
//
// See repeating_uppercase_lowercase_letters_from_string for documentation.

use itertools::Itertools;

fn main() {
  let v = vec![0,1,5,8];
  let j = v.iter()
           .cartesian_product(0..2)
           .map(|x| {
               if let 0=x.1%2 {
                 x.0+10
               } else { 
                 x.0+0
               }
             });
  for x in j {
    println!("{}",x);
  }
}
