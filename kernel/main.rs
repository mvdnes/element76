use core::prelude::*;
use platform::vga::{LightRed, Black};
use kernel::stdio;
use kernel::keyboard;
use kernel::keyboard::{KeyDown, Printable, Space, Escape};

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

	'keyloop: loop
	{
		match keyboard::get_key()
		{
			KeyDown(Printable('7')) => unsafe { asm!("int $$0x23"); },
			KeyDown(Printable('9')) => unsafe { asm!("int $$0x24"); },
			KeyDown(Printable(c)) => { stdio::write_char(0, 0, c, Some(Black), None); },
			KeyDown(Space) => { stdio::write_char(0, 0, ' ', Some(Black), None); },
			KeyDown(Escape) => break 'keyloop,
			_ => {},
		};
	}
}
