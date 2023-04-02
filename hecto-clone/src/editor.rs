use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

pub struct Editor {}

impl Editor{
    pub fn run(&self){
        let _stdout = stdout().into_raw_mode().unwrap();
        
        for key in io::stdin().keys(){
            match key {
                Ok(key) => match key{
                    // Matches any Character and binds it to the variable c
                    Key::Char(c) => {
                        if c.is_control(){
                            // {} is for character known how to print and {:?} is for not 
                            // \r is for carriage return -> Printing the output line by line println!("{:?} \r", b)
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                                }
                        
                    }
                //Specificially matches Ctrl+q 
                Key::Ctrl('q') => break, 
                // Matching every case that is not handled before --> Default case
                _ => println!("{:?}\r", key),
                }, 
                Err(err) => die(err)
                    }
                }
            }



}



