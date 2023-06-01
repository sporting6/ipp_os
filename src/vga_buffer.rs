pub mod color;
pub mod cursor;

use alloc::{
    format,
    string::{String, ToString},
};
use color::{Color, ColorCode};
use core::{error::Error, fmt};
use cursor::Cursor;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use self::cursor::CursorTrait;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Buffer> = Mutex::new(Buffer {
        cursor: Cursor { row: 0, column: 0 },
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut BufferArray) },
    });
}
pub trait VGABuffer {
    // Clears the buffer
    fn clear(&mut self);

    // Clears a row of the buffer
    fn clear_row(&mut self, row: usize);

    // Writes a character with the default color code at cursor's position
    fn write_byte(&mut self, byte: u8);

    // Writes a string at the cursor's position
    fn write_string(&mut self, s: &str);

    // Adds a new line to the buffer by moving all rows up by one, discarding the topmost row
    fn new_line(&mut self);

    // Sets the default color of the buffer
    fn set_color(&mut self, color_code: ColorCode);

    // Returns the position of the cursor
    fn get_cursor(&self) -> (usize, usize);

    // Sets the position of the cursor
    fn set_cursor(&mut self, row: usize, column: usize);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    pub ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

struct BufferArray {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Buffer {
    pub cursor: Cursor,
    buffer: &'static mut BufferArray,
    color_code: ColorCode,
}

impl VGABuffer for Buffer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            byte => {
                if self.cursor.column == BUFFER_WIDTH {
                    self.new_line();
                }
                let color_code = self.color_code;
                self.buffer.chars[self.cursor.row][self.cursor.column].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.cursor.column += 1;
                self.cursor.update();
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            if self.cursor.column >= BUFFER_WIDTH {
                self.new_line();
            }
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        if self.cursor.row >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.cursor.row += 1;
        }
        self.cursor.column = 0;
        self.write_string(" $ ");
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
        self.set_cursor(row, 0);
        self.cursor.update();
    }

    fn clear(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(blank);
            }
        }
        self.set_cursor(0, 0);
        self.write_string(" $ ")
    }

    fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn get_cursor(&self) -> (usize, usize) {
        (self.cursor.row, self.cursor.column)
    }

    fn set_cursor(&mut self, column: usize, row: usize) {
        self.cursor.row = row;
        self.cursor.column = column;
    }
}

impl Buffer {
    pub fn delete_byte(&mut self) {
        if self.cursor.column > 3 {
            self.cursor.column -= 1;
            self.write_byte(b' ');
            self.cursor.column -= 1;
        }
    }

    pub fn run_command(&mut self) {
        let mut row = String::new();
        for i in 3..BUFFER_WIDTH {
            let c = self.buffer.chars[self.cursor.row][i].read().ascii_character;
            row.push(c as char);
        }

        self.command(row)
    }

    fn command(&mut self, s: String) {
        let command = &s.split_whitespace().enumerate().next().unwrap();
        match command.1 {
            "echo" => {
                let echo = &s[4..];
                self.cursor.row += 1;
                self.cursor.column = 0;
                self.write_string(echo);
                self.new_line();
            }
            _ => self.write_string("Incorrect Command"),
        }
    }
}

#[derive(Debug)]
struct InvalidCommandError {
    message: String,
}
impl Error for InvalidCommandError {}

impl fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl InvalidCommandError {
    fn new(message: &str) -> InvalidCommandError {
        InvalidCommandError {
            message: message.to_string(),
        }
    }
}

//  Tests

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
// If stops working: https://os.phil-opp.com/hardware-interrupts/
#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
