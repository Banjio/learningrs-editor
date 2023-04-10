use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
struct Position{
    x: usize,
    y: usize,
}

pub struct Editor {
    // Instead
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor{

    pub fn run(&mut self){
        loop{
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        //We use print to write 4 bytes. The first byte is x1b = Escape character. => This initiates an escape sequence followed by instuctions what to do with the escape sequence
        //print!("\x1b[2j");
        // Using termion we can achieve the same -> Now moved to terminal.rs
        // After clearing the cursor will be at the bottom of the screen this will set it to the top again
        //print!("{}", termion::cursor::Goto(1, 1));
        Terminal::cursor_position(0, 0);
        // Print a goodbye messages in case the user leaves
        // Remember that if we get an error or leave, we once finally refresh the screen => hence we can print it here 
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye, Mate.\r")
        } else {
            self.draw_rows();
            // After darwing rows we will end at the bottom of the screen, this will set our cursor to the top
            //print!("{}", termion::cursor::Goto(1,1));
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        //let pressed_key = Self::read_key()?;
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Greetings Crustacian. The version of this editor is {}.\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        // We do this to center the welcome message according to the terminal size
        let padding = width.saturating_sub(len) / 2;
        // Repeat a string n times
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        // Shorten a string if widh < string length
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height: u16 = self.terminal.size().height;
        for row in 0..height - 1{
            Terminal::clear_current_line();
            if row == height / 3{
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
            
        }
    }

    pub fn default() -> Self {
        let msg = "Mate, initilialising ya terminal failed!";
        Self{
            should_quit: false, 
            // As terminal also can return an error we catch it here and panic. No need to call die, because die would also remove what was drawn to the screen. At this point nothing has been drawn. 
            terminal: Terminal::default().expect(msg),
            // Cursor starts at top left of the screen
            cursor_position: Position { x: 0, y: 0 },
        }
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


