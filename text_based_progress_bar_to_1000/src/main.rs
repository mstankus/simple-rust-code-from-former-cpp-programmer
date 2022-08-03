use std::{thread, time}; // for slowing down the loop
use indicatif::ProgressBar;
// See https://docs.rs/indicatif/latest/indicatif/index.html

fn main() {
  let bar = ProgressBar::new(1000);
  let ten_millis = time::Duration::from_millis(10);
  for _ in 0..1000 {
    bar.inc(1);
    // The next line can be replaced with any code
    // where you want to record the progress
    thread::sleep(ten_millis);
  }
  bar.finish();
} 
