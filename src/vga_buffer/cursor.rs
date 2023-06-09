use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

pub trait CursorTrait {
    // Updates Cursor Position On VGA Buffer
    fn update(&self);

    // Enables the cursor
    fn enable(&self, cursor_start: u32, cursor_end: u32);

    // Disables the cursor
    fn disable(&self);

    // Gets the position of the cursor on the vga buffer
    fn get_vga_position(&self);
}

pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

impl CursorTrait for Cursor {
    fn update(&self) {
        let mut port1: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3D4);
        let mut port2: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3D5);

        let pos: u16 = (self.row * (super::BUFFER_WIDTH as usize) + self.column) as u16;

        unsafe {
            port1.write(0x0F as u8);
            port2.write((pos & 0xFF) as u8);
            port1.write(0x0E as u8);
            port2.write(((pos >> 8) & 0xFF) as u8);
        };
    }

    fn enable(&self, cursor_start: u32, cursor_end: u32) {
        let mut port1: PortGeneric<u32, ReadWriteAccess> = Port::new(0x3D4);
        let mut port2: PortGeneric<u32, ReadWriteAccess> = Port::new(0x3D5);

        unsafe {
            port1.write(0x0A as u32);
            let x = port2.read();
            port2.write((x & 0xC0 as u32) | cursor_start);

            port1.write(0x0B as u32);
            let y = port2.read();
            port2.write((y & 0xE0 as u32) | cursor_end);
        }
    }

    fn disable(&self) {
        todo!()
    }

    fn get_vga_position(&self) {
        todo!()
    }
}
