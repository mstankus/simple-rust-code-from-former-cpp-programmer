fn main() {
  println!("Pair up the values for Vec.");
  let a = vec![1,2,3];
  let b = vec![10,20,30];
  for (a_value,b_value) in a.iter().zip(b.iter()) {
    println!("{} {}",a_value,b_value);
  }
  println!();

  println!("Reverse a list.");
  // Reverse iterator.
  let a = vec![1,2,3];
  for a_value in a.iter().rev() {
    println!("{}",a_value);
  }
  println!();

  println!("Pair up the values for String.");
  let c = "good";
  let d = "evil";
  for (c_value,d_value) in c.chars().zip(d.chars()) {
    println!("{} {}",c_value,d_value);
  }
  println!();

  println!("Pair up the values for String when one shorter.");
  // The assignment to "c" and "d" is called shadowing.
  // Won't see the "d" of good because d is shorter
  let c = "good";
  let d = "bad";
  for (c_value,d_value) in c.chars().zip(d.chars()) {
    println!("{} {}",c_value,d_value);
  }

}
