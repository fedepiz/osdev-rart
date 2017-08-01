use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;

lazy_static ! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.interrupts[0].set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
            println!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
    }
