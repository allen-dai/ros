use crate::serial_print;
use crate::qemu::QemuExitCode;
use crate::qemu::exit_qemu;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...", core::any::type_name::<T>());
        self();
        serial_print!("[ok]\n");
    }
}


pub fn runner(tests: &[&dyn Testable]) {
    serial_print!("\nRunning {} tests\n\n", tests.len());
    for test in tests {
        test.run();
    }
    serial_print!("\n");

    exit_qemu(QemuExitCode::Success);
}
