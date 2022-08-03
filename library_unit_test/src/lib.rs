// Execute "cargo test" to see what happens.

// This is from the internet. No license.
//
// This is a lib.rs file and no main function is needed.
// Write unit tests to test the correctness of library code.

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  // The function add is defined outside of this inline 
  // module and so we want a use command.
  use super::*; 

  #[test]
  fn it_works() {
      let result = add(2, 2);
      assert_eq!(result, 4);
  }
}
