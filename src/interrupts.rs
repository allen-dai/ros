use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/*
   INT_NUM  Short Description PM[clarification needed]
   ------------------------------------------------------------------------------
      0x00 	Division by zero
      0x01 	Single-step interrupt (see trap flag)
      0x02 	NMI
      0x03 	Breakpoint (which benefits from the shorter 0xCC encoding of INT 3)
      0x04 	Overflow
      0x05 	Bound Range Exceeded
      0x06 	Invalid Opcode
      0x07 	Coprocessor not available
      0x08 	Double Fault
      0x09 	Coprocessor Segment Overrun (386 or earlier only)
      0x0A 	Invalid Task State Segment
      0x0B 	Segment not present
      0x0C 	Stack Segment Fault
      0x0D 	General Protection Fault
      0x0E 	Page Fault
      0x0F 	reserved
      0x10 	x87 Floating Point Exception
      0x11 	Alignment Check
      0x12 	Machine Check
      0x13 	SIMD Floating-Point Exception
      0x14 	Virtualization Exception
      0x15 	Control Protection Exception (only available with CET)
*/

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
