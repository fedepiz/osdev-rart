use spin::Mutex;
use port::Port;

struct KeyboardHandlerState {
    pressed:bool,
    shift_down: bool,
}

impl KeyboardHandlerState {
    const fn new() -> KeyboardHandlerState {
        KeyboardHandlerState {
            pressed: false,
            shift_down: false,
        }
    }
}

static PORT:Mutex<Port> = Mutex::new(unsafe { Port::new(0x60) });
static HANDLER_STATE:Mutex<KeyboardHandlerState> = Mutex::new(KeyboardHandlerState::new());

pub fn handle() {
    let mut state = &mut HANDLER_STATE.lock();
    let mut port = &mut PORT.lock();
    let scancode = port.read_byte();
    //If released...
    if scancode & 0x80 != 0 {
        state.pressed = false;
    } else {
        //Pressed...
        //Ignore repeats
        if(!state.pressed) {
            let c = KEYMAP_US_LOWER[(scancode & (!0x80)) as usize];
            print!("{}", c);
            state.pressed = true;
        }
    }
}

const KEYMAP_US_LOWER:[char; 90] = [
'\0' as char,  27 as char, '1', '2', '3', '4', '5', '6', '7', '8',	/* 9 */
'9', '0', '-', '=', '\x08',	/* Backspace */
'\t',			/* Tab */
'q', 'w', 'e', 'r',	/* 19 */
't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',	/* Enter key */
  '\0',			/* 29   - Control */
'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';',	/* 39 */
'\'', '`',   '\0',		/* Left shift */
'\\', 'z', 'x', 'c', 'v', 'b', 'n',			/* 49 */
'm', ',', '.', '/',   '\0',				/* Right shift */
'*',
  '\0',	/* Alt */
' ',	/* Space bar */
  '\0',	/* Caps lock */
  '\0',	/* 59 - F1 key ... > */
  '\0',   '\0',   '\0',   '\0',   '\0',   '\0',   '\0',   '\0',
  '\0',	/* < ... F10 */
  '\0',	/* 69 - Num lock*/
  '\0',	/* Scroll Lock */
  '\0',	/* Home key */
  '\0',	/* Up Arrow */
  '\0',	/* Page Up */
'-',
  '\0',	/* Left Arrow */
  '\0',
  '\0',	/* Right Arrow */
'+',
  '\0',	/* 79 - End key*/
  '\0',	/* Down Arrow */
  '\0',	/* Page Down */
  '\0',	/* Insert Key */
  '\0',	/* Delete Key */
  '\0',   '\0',   '\0',
  '\0',	/* F11 Key */
  '\0',	/* F12 Key */
  '\0'  	/* All other keys are undefined */
];
