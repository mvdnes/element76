use core::prelude::*;
use platform::vga::{write_screen, LightRed, Black, Yellow, LightCyan};
use platform::keyboard;
use platform::keyboard::{KeyDown, Printable, Alt};

#[no_mangle]
#[no_split_stack]
pub fn start()
{
	main();
	::platform::cpu::halt();
}

fn main()
{
	for x in range(0u, 80)
	{
		for y in range(0u, 25)
		{
			write_screen(x, y, ' ' as u8, None, Some(LightRed));
		}
	}

	let hw = " Hello, World! ";
	for i in range(0u, hw.len())
	{
		write_screen(3+i, 3, hw[i],
		Some(match hw[i]
		{
			0x61..0x7A => LightCyan, // Lowercase ASCII
			_ => Yellow,
		}), Some(Black));
		write_screen(5+i, 5, hw[i], Some(Black), None);
	}

	'keyloop: loop
	{
		match keyboard::get_key()
		{
			KeyDown(Printable(c)) => write_screen(0, 0, c as u8, Some(Black), None),
			KeyDown(Alt) => break 'keyloop,
			_ => {},
		};
	}
}
