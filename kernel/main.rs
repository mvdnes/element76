use platform::vga::{LightRed, Black, Yellow};
use kernel::stdio::StdioWriter;

#[no_mangle]
pub fn start()
{
	::platform::cpu::setup();
	main();
	loop { ::platform::cpu::halt(); }
}

fn main()
{
	let mut printer = StdioWriter::new();
	printer.bg = LightRed;
	printer.fg = Yellow;
	printer.clear_screen();
	printer.fg = Black;
	printer.move(3, 3);
	printer.print_screen("Hello, World!");
}
