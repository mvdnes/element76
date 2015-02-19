use platform::vga::Color;
use kernel::stdio::StdioWriter;
use core::fmt::Write;

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

#[lang = "panic_fmt"]
extern fn panic_fmt(args: ::core::fmt::Arguments, file: &str, line: u32) -> !
{
	let mut printer = StdioWriter::new();
	printer.bg = Color::Black;
	printer.fg = Color::Red;

	printer.print_screen("RUST FAIL");
	printer.crlf();

	let _ = printer.write_fmt(args);
	printer.crlf();

	printer.print_screen(file);
	printer.print_char(':');
	printer.print_dec(line);

	::platform::cpu::halt();
}
