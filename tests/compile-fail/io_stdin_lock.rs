// "For technical reasons, `io::stdin().lock()` doesn't work."

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin_locked = io::stdin().lock();
    //~^ERROR: borrowed value does not live long enough

    for line in stdin_locked.lines() {
        println!("*** {}", line.unwrap());
    }
}
