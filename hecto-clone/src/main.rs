use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes(){
        // let c:u8 = b.unwrap(); -> If you do it like this the integer representations of chars are returned
        let c:char = b.unwrap() as char;
        println!("{}", c);
    }
}
