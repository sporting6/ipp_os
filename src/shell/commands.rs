use core::fmt::Write;

use alloc::{vec::Vec, string::String};

use crate::vga_buffer::{WRITER, VGABuffer};

use super::{SHELL};

pub fn echo(args: Vec<&str>) -> Result<(), &'static str> {
    let mut to_echo = String::new();
    for s in args{
        to_echo.push(' ');
        to_echo.push_str(&s);
    }

    let mut writer = WRITER.lock();
    writer.new_line();
    writer.write_string(&to_echo);
    
    Ok(())
}

pub fn cowsay(args: Vec<&str>) -> Result<(), &'static str> {
    let mut message = String::new();
    for s in args{
        message.push(' ');
        message.push_str(&s);
    }

    let message_lines = message.lines().collect::<Vec<_>>();
    let message_width = message_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let bubble_width = message_width + 2;
    let mut horizontal_line = String::from("\n ");
    horizontal_line.push_str(&"-".repeat(bubble_width));

    let mut writer = WRITER.lock();

    writer.write_string(&horizontal_line);
    for (i, line) in message_lines.iter().enumerate() {
        let mut s = String::from("\n| ");
        s.push_str(line);
        s.push_str(&" ".repeat(message_width - line.len()));
        s.push_str(" |");
        writer.write_string(&s);
    }

    writer.write_string(&horizontal_line);
    writer.write_string("\n        \\   ^__^");
    writer.write_string("\n         \\  (oo)\\_______");
    writer.write_string("\n            (__)\\       )\\/\\");
    writer.write_string("\n                ||----w |");
    writer.write_string("\n                ||     ||");
    Ok(())
}
