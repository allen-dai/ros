use core::panic::PanicInfo;

pub fn test_panic(info: &PanicInfo) -> ! {
    use crate::qemu::exit_qemu;
    use crate::qemu::QemuExitCode;
    use crate::serial_println;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
