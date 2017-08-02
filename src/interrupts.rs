use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;
use pic;
lazy_static ! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.interrupts[0].set_handler_fn(pic_handler);
        idt.interrupts[1].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
            println!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, errCode:u64) {
    println!("Exception: Double fault!\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn pic_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("PIT trigger");
    unsafe {
        pic::signal_irq_done(0);
    }
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    unsafe {
        pic::signal_irq_done(1);
    }
}
