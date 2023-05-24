use alloc::{string::String};
use crate::vga_buffer::{VGABuffer, Buffer, color::{Color, ColorCode}};
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

#[macro_export]
macro_rules! shell_print {
    ($($arg:tt)*) => ($crate::shell::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! shell_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        color: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        line_start: String::from(" $ "),
        cursor_position: (0, 0),
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    
    interrupts::without_interrupts(|| {
        SHELL.lock().write_fmt(args).unwrap();
    });
}

impl fmt::Write for Shell {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub struct Shell {
    buffer: &'static mut Buffer, // the VGA Buffer
    line_start: String, // What is printed before where you can type
    color: ColorCode, // Color of the fg and bg
    cursor_position: (usize, usize), // Where on the buffer the cursor is (column, row)
}

impl Shell {
    pub fn init(&mut self){
        self.clear();
        self.cursor_position = (self.buffer.columns(), self.buffer.rows());
        let line_start = self.line_start.clone();
        self.write_string(&line_start);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buffer.write_byte(self.cursor_position.1, self.cursor_position.0, byte);
    }
    
    pub fn delete_byte(&mut self) {
        if (self.cursor_position.0 > self.line_start.as_bytes().len()){
            self.cursor_position.0 -= 1;
            self.buffer.write_byte(self.cursor_position.1, self.cursor_position.0, b' ');
        }   
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn set_color(&mut self, color: ColorCode){
        self.color = color;
    }

    pub fn new_line(&mut self){
        self.buffer.new_line();
        self.cursor_position.1 +=1;
    }

    pub fn update(&mut self) {
        self.new_line();
        let line_start = self.line_start.clone();
        self.write_string(&line_start);
        self.cursor_position.0 = line_start.as_bytes().len();
    }

    pub fn set_line_start(&mut self, s: String) {
        self.line_start = s;
    }

    fn clear(&mut self){
        self.buffer.clear();
    }
    
}