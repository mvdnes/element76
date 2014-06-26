use platform::vga::{Red, White, Yellow, Black};
use kernel::stdio::StdioWriter;
use core::fmt::FormatWriter;

#[no_mangle]
pub fn entry()
{
	::platform::cpu::setup();
	main();
	loop { ::platform::cpu::idle(); }
}

fn main()
{
	let mut printer = StdioWriter::new();
	printer.bg = Red;
	printer.fg = Yellow;
	printer.clear_screen();
	printer.fg = White;
	printer.move(3, 3);
	printer.print_screen("Hello, World!");
}

#[lang = "begin_unwind"]
extern fn begin_unwind(args: &::core::fmt::Arguments, file: &str, line: uint) -> !
{
	let mut printer = StdioWriter::new();
	printer.bg = Black;
	printer.fg = Red;

	printer.print_screen("RUST FAIL");
	printer.crlf();

	let _ = printer.write_fmt(args);
	printer.crlf();

	printer.print_screen(file);
	printer.print_char(':');
	printer.print_dec(line);

	::platform::cpu::halt();
}
