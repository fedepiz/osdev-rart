use port::Port;
use spin::Mutex;
use core::fmt;


const COM1:u16 = 0x3F8;

pub struct SerialPort {
    io_port: Port,
}

impl SerialPort {
    const unsafe fn new(address:u16) -> SerialPort {
        SerialPort {
            io_port: Port::new(address),
        }
    }

    unsafe fn initialize(&mut self) {
        /*self.io_port.write_byte_offset(1, 0x00);
        self.io_port.write_byte_offset(3, 0x80);
        self.io_port.write_byte_offset(0, 0x03);
        self.io_port.write_byte_offset(1, 0x00);
        self.io_port.write_byte_offset(3, 0x03);
        self.io_port.write_byte_offset(2, 0xC7);
        self.io_port.write_byte_offset(4, 0x0B);*/
    }

    unsafe fn wait_input_received(&mut self) {
        let mut done = false;
        while !done {
            done = self.io_port.read_byte_offset(5) & 1 != 0;
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe {
            self.wait_input_received();
            self.io_port.read_byte()
        }
    }

    unsafe fn wait_can_write(&mut self) {
        let mut done = false;
        while !done {
            done = self.io_port.read_byte_offset(5) & 0x20 != 0;
        }
    }

    pub fn write_byte(&mut self, value:u8) {
        unsafe {
            self.wait_can_write();
            self.io_port.write_byte(value);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

pub static SERIAL1:Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(COM1) });

pub unsafe fn initialize() {
    SERIAL1.lock().initialize();
}

pub fn log(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).unwrap();
}
