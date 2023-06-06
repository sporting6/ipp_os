use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use crate::vga_buffer::{VGABuffer, WRITER};


pub fn help(_args: Vec<String>) -> Result<(), &'static str> {
    let mut writer = WRITER.lock();
    writer.write_string("\nPossible Commands:");
    writer.write_string("\necho args");
    writer.write_string("\nrps action");
    writer.write_string("\ncowsay args");
    writer.write_string("\ncalc num1 operator num2");

    Ok(())
}



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

pub fn rockpaperscissors(args: Vec<String>) -> Result<(), &'static str> {
    let p_choice = match args.get(0).map(|s| &**s) {
        Some("rock") => 0,
        Some("paper") => 1,
        Some("scissors") => 2,
        _ => return Err("Invalid Argument"),
    };


    let c_choice: u16 = 1;

    WRITER.lock().write_string("\n 3 ");
    delay_one_second();
    WRITER.lock().write_string(" 2 ");
    delay_one_second();
    WRITER.lock().write_string(" 1 ");

    WRITER.lock().write_string(&format!("\n You Chose: {}",match p_choice{
        0 => "Rock",
        1 => "Paper",
        2 => "Scissors",
        _ => "",
    }));

    WRITER.lock().write_string(&format!("\n I Chose: {}", match c_choice {
        0 => "Rock",
        1 => "Paper",
        2 => "Scissors",
        _ => "",
    }));


    Ok(())
}

fn delay_one_second() {
    // Adjust this value based on your system's clock speed and desired accuracy
    const LOOP_COUNT: u32 = 500_000;

    for _ in 0..LOOP_COUNT {
        // Perform some non-optimizeable operation to occupy the CPU
        // Here, we use a volatile read from memory to prevent the loop from being optimized away
        unsafe {
            core::ptr::read_volatile(&0);
        }
    }
}


