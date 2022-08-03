// This file is from the internet. No license.

use chrono::Utc;

// Global variables are bad, of course, but acceptable with thread safety. 
// This helps about a (Computer Science) race condition.

thread_local!(static GLOBAL_DATA: String = Utc::now().to_string());

fn main() {
    GLOBAL_DATA.with(|text| {
        println!("{}", *text);
    });
}
