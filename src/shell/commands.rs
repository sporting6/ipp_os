use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::vga_buffer::{VGABuffer, WRITER};

pub fn echo(args: Vec<String>) -> Result<(), &'static str> {
    let to_echo = match args.get(0) {
        Some(s) => s,
        None => "",
    };

    let mut writer = WRITER.lock();
    writer.new_line();
    writer.write_string(&to_echo);

    Ok(())
}

const BUBBLE_WIDTH: usize = 30;

pub fn cowsay(args: Vec<String>) -> Result<(), &'static str> {
    let message: Vec<String> = match args.get(0) {
        Some(s) => {
            let mut to_return: Vec<String> = Vec::new();
            let mut current_line = String::new();

            for word in s.split_whitespace() {
                if current_line.len() + word.len() + 1 <= BUBBLE_WIDTH {
                    current_line.push_str(word);
                    current_line.push(' ');
                } else {
                    to_return.push(current_line.trim().to_string());
                    current_line = String::from(word);
                    current_line.push(' ');
                }
            }

            if !current_line.is_empty() {
                to_return.push(current_line.trim().to_string());
            }

            to_return
        }
        None => return Err("Invalid Message"),
    };

    let horizontal_line = format!("\n {}", "-".repeat(BUBBLE_WIDTH));

    let mut writer = WRITER.lock();

    writer.write_string(&horizontal_line);
    for line in message {
        writer.write_string(&format!(
            "\n| {}{} |",
            line,
            " ".repeat(BUBBLE_WIDTH - line.len() - 2)
        ))
    }

    writer.write_string(&horizontal_line);
    writer.write_string("\n        \\   ^__^");
    writer.write_string("\n         \\  (oo)\\_______");
    writer.write_string("\n            (__)\\       )\\/\\");
    writer.write_string("\n                ||----w |");
    writer.write_string("\n                ||     ||");

    Ok(())
}

pub fn calc(args: Vec<String>) -> Result<(), &'static str> {
    if args.len() != 3 {
        return Err("Invalid number of arguments");
    }

    let num1: i32 = match args[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number"),
    };

    let operator = &args[1];

    let num2: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number"),
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0 {
                return Err("Division by zero");
            }
            num1 / num2
        }
        _ => return Err("Invalid operator"),
    };

    WRITER.lock().write_string(&format!("\nResult: {}", result));

    Ok(())
}
