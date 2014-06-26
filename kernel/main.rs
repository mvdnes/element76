use platform::vga::{Red, White, Yellow};
use kernel::stdio::StdioWriter;

#[no_mangle]
pub fn entry()
{
	::platform::cpu::setup();
	main();
	loop { ::platform::cpu::halt(); }
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
