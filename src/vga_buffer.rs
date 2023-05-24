pub mod color;

use color::{Color, ColorCode};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

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
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut BufferArray) },
    });
}
pub trait VGABuffer {
    // Clears the buffer
    fn clear(&mut self);

    // Clears a row of the buffer
    fn clear_row(&mut self, row: usize);

    // Writes a character with the default color code at the specified position in the buffer
    fn write_byte(&mut self, row: usize, col: usize, byte: u8);

    // Adds a new line to the buffer by moving all rows up by one, discarding the topmost row
    fn new_line(&mut self);

    // Returns the number of rows in the buffer
    fn rows(&self) -> usize {
        BUFFER_HEIGHT
    }

    // Returns the number of columns in the buffer
    fn columns(&self) -> usize {
        BUFFER_WIDTH
    }
    // Sets the color of the buffer
    fn set_color(&mut self, color_code: ColorCode);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct BufferArray {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Buffer {
    column_position: usize, //temp
    buffer: &'static mut BufferArray,
    color_code: ColorCode,
}

impl VGABuffer for Buffer {
    fn write_byte(&mut self, row: usize, col: usize, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' => self.delete_byte(),
            byte => {
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..self.rows() {
            for col in 0..self.columns() {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(self.rows() - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn clear(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..self.columns() {
            for row in 0..self.rows() {
                self.buffer.chars[row][col].write(blank);
            }
        }
    }

    fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }
}

impl Buffer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(self.rows() - 1, self.column_position, byte),
                // not part of printable ASCII range
                _ => self.write_byte(self.rows() - 1, self.column_position, 0xfe),
            }
        }
    }

    pub fn delete_byte(&mut self) {
        if (self.column_position > 0) {
            self.column_position -= 1;
            self.write_byte(self.rows() - 1, self.column_position, b' ');
            self.column_position -= 1;
        }
    }
}

//  Tests

#[test_case]
fn test_println_simple() {
    // println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        // println!("test_println_many output");
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
