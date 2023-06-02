use alloc::{string::String, vec::Vec};

use crate::vga_buffer::{VGABuffer, WRITER, BUFFER_WIDTH};

pub fn run_command() {
    let mut row = String::new();
    for i in 3..BUFFER_WIDTH {
        let writer = WRITER.lock();
        let c = writer.buffer.chars[writer.cursor.row][i].read().ascii_character;
        row.push(c as char);
    }

    parse(row)
}

fn parse(s: String) {
    let mut args: Vec<&str> = s.split_whitespace().collect(); 
    let command = args.get(0).copied();  

    args.remove(0);

    if let Some(first_word) = command {
        match first_word {
            "echo" => echo(args),
            _ => WRITER.lock().write_string("Incorrect Command\n"),
        }
    }
}

pub fn echo(args: Vec<&str>){
    let mut to_echo = String::new();
    for s in args{
        to_echo.push(' ');
        to_echo.push_str(&s);
    }
    let mut writer = WRITER.lock();
    writer.cursor.row += 1;
    writer.cursor.column = 0;
    writer.write_string(&to_echo);
    writer.new_line();
}