// Mark Stankus (c) Wed Aug  3 13:37:37 PDT 2022
// File: main.rs

// Your files which are to be "included" in the 
// sense of C++ should have a "mod" statement.
mod a;
mod b;

fn main() {
  // This function ace is defined in a.rs.
  a::ace();
}
