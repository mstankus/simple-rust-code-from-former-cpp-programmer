fn main() {
  println!("This \x08does \x08not \x08have \x08any \x08spaces.");

  print!("Before");
  print!("\x08\x08\x08\x08\x08\x08");
  println!("After 1");

  print!("Before");
  for _i in 0.."Before".chars().count() {
    print!("{}",'\x08');
  }
  println!("After 2");
}
