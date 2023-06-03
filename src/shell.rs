pub mod commands;

use alloc::{string::String, vec::Vec};
use spin::Mutex;

use crate::{vga_buffer::{VGABuffer, WRITER, BUFFER_WIDTH}, println};
use lazy_static::lazy_static;

use self::commands::{echo, cowsay};

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell { 
        prompt: String::from(" $ "),
        active: false,
    });
}

pub struct Shell{
    prompt: String,
    active: bool,
}

impl Shell {
    pub fn write_prompt(&self) {
        if self.active {
            let writer = &mut WRITER.lock();
            writer.write_string(&self.prompt);
        }
    }

    pub fn run_command(&self) -> Result<(), &'static str>{
        if self.active {
            match parse(get_row().expect("Invalid Row")){
                Ok(ok) => return Ok(ok),
                Err(e) => return Err(e),
            }
        }
        else {
            Err("Error Running Command")
        }
    }

    pub fn start_shell(&mut self) -> Result<(), &'static str>{
        println!("Loading Shell....");
        self.active = true;

        {
            let mut writer = WRITER.lock();
            writer.clear();
            writer.write_string(&self.prompt);
        }

        Ok(())
    }
}



fn get_row() -> Result<String, &'static str> {
    let mut row = String::new();
    for i in 3..BUFFER_WIDTH {
        let writer = WRITER.lock();
        let c = writer.buffer.chars[writer.cursor.row][i].read().ascii_character;
        row.push(c as char);
    };
    Ok(row)
}

fn parse(s: String) -> Result<(), &'static str> {
    let mut args: Vec<&str> = s.split_whitespace().collect(); 
    if args .len() > 0 {
        let command = args.get(0).copied();  

        args.remove(0);
    
        if let Some(first_word) = command {
            let mut to_return;
            match first_word {
                "echo" => to_return = echo(args),
                "cowsay" => to_return = cowsay(args),
                _ => to_return = Err("Invalid Command"),
            }
            return to_return;
        } else {
            Err("Invalid Command")
        }
    } else {
        Err("Invalid Command")
    }
}