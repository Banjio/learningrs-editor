mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;
fn main(){
    //let mut editor = Editor::default();
    //editor.run();
    Editor::default().run();
}

// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     // Ctrl sets the first 3 bits to zero compared to the same character without ctrl
//     byte & 0b0001_1111
// }

// fn main() {
//     // We ignore the error that may occur from .into_raw_mode()
//     let _stdout = stdout().into_raw_mode().unwrap();
//     for b in io::stdin().bytes(){
//         match b {
//             Ok(b) => {
//                 // As => primitive to single (byte to in this case char)
//                 let c = b as char;
//                 // Test wheter a character is a control character, based on ASCII Representations
//                 if c.is_control(){
//                     // {} is for character known how to print and {:?} is for not 
//                     // \r is for carriage return -> Printing the output line by line println!("{:?} \r", b)
//                     println!("{:?} \r", b);
//                 } else {
//                     println!("{:?} ({})\r", b, c);
//                 }
//                 // to_ctrl_byte('q') does the same as ctrl in ASCII -> setting the first numbers of a characters
//                 // byte representation to its CTRL equivalent
//                 if b == to_ctrl_byte('q'){
//                     break
//                 }
//             }
//             Err(err) => die(err)
//         }
//     }
// }
