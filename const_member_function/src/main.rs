struct Animal {
  age: usize,
}

// The member functions are don't change self.
// The member functions are public within this file.
// The member functions are private when used from outside this file.

impl Animal {
  fn age_now(&self) -> usize {
    self.age
  }
  fn age_next_year(&self) -> usize {
    self.age+1
  }
}

fn main() {
  let x = Animal { age: 3};
  println!("Age this year {}",x.age);
  println!("Age this year {}",x.age_now());
  println!("Age next year {}",x.age_next_year());
}
