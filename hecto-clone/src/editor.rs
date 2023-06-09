use crate::Terminal;
use crate::Document;
use crate::Row;
use termion::event::Key;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
#[derive(Default)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    // Instead
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position, 
    document: Document,
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
        Terminal::cursor_position(&Position::default());
        // Print a goodbye messages in case the user leaves
        // Remember that if we get an error or leave, we once finally refresh the screen => hence we can print it here 
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye, Mate.\r");
        } else {
            self.draw_rows();
            // After darwing rows we will end at the bottom of the screen, this will set our cursor to the top
            //print!("{}", termion::cursor::Goto(1,1));
            Terminal::cursor_position(&Position { 
                x: self.cursor_position.x.saturating_sub(self.offset.x), 
                y: self.cursor_position.y.saturating_sub(self.offset.y)
                }
            );
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        //let pressed_key = Self::read_key()?;
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up 
                | Key::Down 
                | Key::Left 
                | Key::Right
                | Key::PageUp
                | Key::PageDown
                | Key::End
                | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self){
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y
        } else if y >= offset.y.saturating_add(height) {
        offset.y = y.saturating_sub(height).saturating_add(1)
        }
        if x < offset.x {
            offset.x = x
        } else if x >= offset.x.saturating_add(width){
            offset.x = x.saturating_sub(width).saturating_add(1)
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position{mut y, mut x} = self.cursor_position;
        //let size = self.terminal.size();
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height{
                    y = y.saturating_add(1);
                }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),

        }
        self.cursor_position = Position {x, y}
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

    pub fn draw_row(&self, row: &Row){
        // let start = 0;
        // let end = self.terminal.size().width as usize;
        // let row = row.render(start, end);
        // println!("{}\r", row)
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height: u16 = self.terminal.size().height;
        for terminal_row in 0..height - 1{
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y){
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3{
                self.draw_welcome_message()
            } else {
                println!("~\r");
            }
            
        }
    }

    pub fn default() -> Self {
        let msg = "Mate, initilialising ya terminal failed!";
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1{
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        Self{
            should_quit: false, 
            // As terminal also can return an error we catch it here and panic. No need to call die, because die would also remove what was drawn to the screen. At this point nothing has been drawn. 
            terminal: Terminal::default().expect(msg),
            // Cursor starts at top left of the screen
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
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


