use port::Port;
use spin::Mutex;

struct Pit {
    port: Port,
    divisor: u32,
}

impl Pit {
    const unsafe fn new() -> Pit {
        Pit {
            port:Port::new(0x40),
            divisor: 0,
        }
    }
    pub fn divisor(self) {
        self.divisor;
    }
    fn set_timer_phase(&mut self, hz:u32) {
        let divisor:u32 = 1193180 / hz;
        logln!("Numbers are {}, {} and {}", 1193180, hz, divisor);
        self.divisor = divisor;
        self.port.write_byte_offset(3, 0x36);
        self.port.write_byte((divisor & 0xFF) as u8);
        self.port.write_byte((divisor >> 8) as u8);
    }
}

pub struct PitHandlerState {
    ticks_killed: u32,
    ticks_per_trigger: u32,
}

impl PitHandlerState {
    const fn new(ticks_per_trigger: u32) -> PitHandlerState {
        PitHandlerState {
            ticks_killed: 0,
            ticks_per_trigger: ticks_per_trigger,
        }
    }

    pub fn should_trigger(&self) -> bool {
        self.ticks_killed == self.ticks_per_trigger
    }

    pub fn reset_killed_count(&mut self) {
        self.ticks_killed = 0;
    }

    pub fn kill_tick(&mut self) {
        self.ticks_killed += 1;
    }
}

fn default_handler(state: &PitHandlerState) {
    println!("Triggered!");
}


static PIT:Mutex<Pit> = Mutex::new(unsafe { Pit::new() });
pub static PIT_HANDLER:Mutex<(PitHandlerState, fn(&PitHandlerState) ->())> =
    Mutex::new((PitHandlerState::new(1800), default_handler));

pub fn initialize() {
    PIT.lock().set_timer_phase(1000);
}
