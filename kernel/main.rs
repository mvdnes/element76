use crate::platform::vga::Color;
use crate::kernel::stdio::StdioWriter;
use crate::platform;
use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub fn entry() -> !
{
    platform::cpu::setup();
    platform::mmu::setup();
    platform::cpu::enable_interrupts();
    main();
    loop { platform::cpu::idle(); }
}

fn main()
{
    let mut printer = StdioWriter::new();
    printer.bg = Color::Red;
    printer.fg = Color::Yellow;
    printer.clear_screen();
    printer.fg = Color::White;
    printer.go_to(3, 3);
    printer.print_screen("Hello, World!");
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> !
{
    let mut printer = StdioWriter::new();
    printer.bg = Color::Black;
    printer.fg = Color::Red;

    printer.print_screen("RUST FAIL");
    printer.crlf();

    if let Some(location) = info.location() {
        let _ = writeln!(printer, "Panic in '{}' at line {}", location.file(), location.line());
    }
    else {
        let _ = writeln!(printer, "Panic at unknown location");
    }
    let _ = write!(printer, "{}", info.message());

    platform::cpu::halt();
}
