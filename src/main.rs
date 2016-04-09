#[macro_use]
extern crate lazy_static;

extern crate termios;

use std::str;

mod term;

fn main() {
    let mut term = term::TermWrap::stdin();

    let quit = "q".as_bytes();
    loop {
        let input = term.read();

        if input != &[] {
            unsafe { println!("{}, {:?}", str::from_utf8_unchecked(input), input) };

            if input == quit {
                break
            }
        }
    }
}
