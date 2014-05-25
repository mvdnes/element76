use platform::vga::{LightRed, Black};
use kernel::stdio;

#[no_mangle]
#[no_split_stack]
pub fn start()
{
	::platform::cpu::setup();
	main();
	loop { ::platform::cpu::halt(); }
}

#[no_split_stack]
fn main()
{
	stdio::set_bg(LightRed);
	stdio::set_fg(Black);
	stdio::clear_screen();
	stdio::write_screen(3, 3, "Hello, World!");
	stdio::move(0, 4);
}
