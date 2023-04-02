use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

pub struct Editor {
    // Instead
    should_quit: bool,
}

impl Editor{

    pub fn run(&mut self){
        let _stdout = stdout().
        into_raw_mode().
        unwrap();
        loop{
            if let Err(error) = self.process_keypress(){
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            // io::stdin().lock().keys().next() returns an Option which also can be None in which case the loop is continued
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn default() -> Self {
        Self{should_quit: false}
    }
}

/* impl Editor{
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
                _ => println!("{key:?}\r"),
                }, 
                Err(err) => die(err)
                    }
                }
            }
    
    pub fn default() -> Self {
        Self{}
    }



}
 */


