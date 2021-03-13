use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    // Enter raw mode
    let _stdout = stdout().into_raw_mode().unwrap();
    // Read the input
    for b in io::stdin().bytes() {
        //* variable shadowing
        match b {
            Ok(b) => {
                let c = b as char;
                // check if `c` is a control character
                // * control character = non-printable character -- ie. ASCII 0-31
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
