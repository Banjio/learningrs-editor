use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    // Ctrl sets the first 3 bits to zero compared to the same character without ctrl
    byte & 0b0001_1111
}

fn die(e: std::io::Error){
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes(){
        let b = b.unwrap();
        // As => primitive to single (byte to in this case char)
        let c:char = b as char;
        // Test wheter a character is a control character, based on ASCII Representations
        if c.is_control(){
            // {} is for character known how to print and {:?} is for not known characters which may have a debug representation
            // \r is for carriage return -> Printing the output line by line
            println!("{:?} \r", b)
        } else {
            println!("{:?} ({}) \r", b, c)
        }
        // to_ctrl_byte('q') does the same as ctrl in ASCII -> setting the first numbers of a characters
        // byte representation to its CTRL equivalent
        if b == to_ctrl_byte('q'){
            break;
        }
    }
}
