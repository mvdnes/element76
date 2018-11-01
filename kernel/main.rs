use platform::vga::Color;
use kernel::stdio::StdioWriter;
use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub fn entry() -> !
{
    ::platform::cpu::setup();
    ::platform::mmu::setup();
    ::platform::cpu::enable_interrupts();
    main();
    loop { ::platform::cpu::idle(); }
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

    match info.message() {
        Some(args) => { let _ = printer.write_fmt(*args); },
        None => { printer.print_screen("<No message provided>"); },
    };

    match info.location() {
        Some(location) => {
            printer.crlf();
            printer.print_screen(location.file());
            printer.print_char(':');
            printer.print_dec(location.line());
            printer.print_char(':');
            printer.print_dec(location.column());
        },
        None => {},
    }

    ::platform::cpu::halt();
}
