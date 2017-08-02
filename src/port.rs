use x86_64::instructions::port::*;

trait PortValueType {
    type ValueType;

}

pub struct Port {
    pub port: u16,
}

impl Port {
    pub const unsafe fn new(port: u16) -> Port {
        Port { port: port }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe { inb(self.port) }
    }

    pub fn read_byte_offset(&mut self, offset:u16) -> u8 {
        unsafe { inb(self.port + offset) }
    }

    pub fn write_byte(&mut self, value: u8) {
        unsafe { outb(self.port, value) }
    }

    pub fn write_byte_offset(&mut self, value:u8, offset:u16) {
        unsafe { outb(self.port + offset, value) }
    }
}
