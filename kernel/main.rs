use core::prelude::*;
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

fn main()
{
	stdio::clear_screen(LightRed);
	stdio::write_screen(3, 3, " Hello, World! ", Some(Black), None);
}
