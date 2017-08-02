use x86_64::instructions::port::outb;
use spin::Mutex;
use port::Port;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

static PICS:Mutex<Pics> = Mutex::new(unsafe { Pics(Pic::new(0x20, 0x21), Pic::new(0x20, 0x21))});

struct Pics(Pic,Pic);

impl Pics {
     unsafe fn remap(&mut self) {
        self.0.write(PortType::Command, 0x11);
        self.1.write(PortType::Command, 0x11);
        self.0.write(PortType::Data, 0x20);
        self.1.write(PortType::Data, 0x28);
        self.0.write(PortType::Data, 0x04);
        self.1.write(PortType::Data, 0x02);
        self.0.write(PortType::Data, 0x01);
        self.1.write(PortType::Data, 0x01);
        self.0.write(PortType::Data, 0x0);
        self.1.write(PortType::Data, 0x0);
    }

    pub unsafe fn signal_irq_done(&mut self, int_no:u8) {
        if(int_no >= 0) {
            self.1.write(PortType::Command, 0x20);
        }
        self.0.write(PortType::Command, 0x20);
    }
}


#[derive(Debug, Copy, Clone)]
enum PortType {
    Command,
    Data,
}

struct Pic {
    cmd_port: Port,
    data_port: Port,
}

impl Pic {
    const unsafe fn new(cmd_addr:u16, data_addr:u16) -> Pic {
        Pic {
            cmd_port: Port::new(cmd_addr),
            data_port: Port::new(data_addr),
        }
    }

    fn portOf(&mut self, portType:PortType) -> &mut Port {
        match portType {
            PortType::Command => &mut self.cmd_port,
            PortType::Data => &mut self.data_port,
        }
    }

    fn write(&mut self, portType:PortType, value:u8) {
        let port = self.portOf(portType);
        port.write_byte(value);
    }
}

pub unsafe fn initialize() {
    PICS.lock().remap();
}

pub unsafe fn signal_irq_done(int_no:u8) {
    PICS.lock().signal_irq_done(int_no);
}
