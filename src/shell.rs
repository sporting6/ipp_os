use alloc::{string::String, vec::Vec};

use crate::vga_buffer::{VGABuffer, WRITER};

pub fn echo(args: Vec<&str>){
    let mut to_echo = String::new();
    for s in args{
        to_echo.push(' ');
        to_echo.push_str(&s);
    }
    
    WRITER.lock().cursor.row += 1;
    WRITER.lock().cursor.column = 0;
    WRITER.lock().write_string(&to_echo);
    WRITER.lock().new_line();
}