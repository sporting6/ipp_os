pub mod commands;

use alloc::{string::String, vec::Vec};
use spin::Mutex;

use crate::{
    println,
    vga_buffer::{VGABuffer, BUFFER_WIDTH, WRITER},
};
use lazy_static::lazy_static;

use self::commands::{cowsay, echo, calc};

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        prompt: String::from(" $ "),
        active: false,
    });
}

pub struct Shell {
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

    pub fn run_command(&self) -> Result<(), &'static str> {
        if self.active {
            match parse(get_command().expect("Invalid Row")) {
                Ok(ok) => return Ok(ok),
                Err(e) => return Err(e),
            }
        } else {
            Err("Error Running Command")
        }
    }

    pub fn start_shell(&mut self) -> Result<(), &'static str> {
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

fn get_command() -> Result<String, &'static str> {
    let mut to_return = String::new();
    let writer = WRITER.lock();

    let start_row: usize = {
        let mut result = 0;
        for i in 0..writer.cursor.row + 1 {
            if writer.buffer.chars[i][1].read().ascii_character == b'$' {
                result = i;
            }
        }
        result
    };

    for i in 3..BUFFER_WIDTH {
        let c = writer.buffer.chars[start_row][i].read().ascii_character;
        to_return.push(c as char);
    }

    if start_row < writer.cursor.row {
        for y in start_row + 1..writer.cursor.row + 1 {
            for x in 0..BUFFER_WIDTH {
                let c = writer.buffer.chars[y][x].read().ascii_character;
                to_return.push(c as char);
            }
        }
    }

    Ok(to_return)
}

fn parse(s: String) -> Result<(), &'static str> {
    let args: Vec<String> = parse_arguments(&s);
    if !args.is_empty() {
        let command = args[0].clone();
        let mut rest = args;
        rest.remove(0);

        if let Some(first_word) = Some(command.as_str()) {
            match first_word {
                "echo" => echo(rest),
                "cowsay" => cowsay(rest),
                "calc" => calc(rest),
                _ => Err("Invalid Command"),
            }
        } else {
            Err("Invalid Command")
        }
    } else {
        Err("Invalid Command")
    }
}

fn parse_arguments(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            ' ' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}
