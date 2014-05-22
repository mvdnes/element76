use core::prelude::*;
use platform::vga::{write_screen, LightRed, Black, Yellow, LightCyan};

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

	loop
	{
		let code = unsafe { ::platform::io::inport(0x60) };
		if code & 0x80 == 0
		{
			let ccode = match code
			{
				 2 => '1',
				 3 => '2',
				 4 => '3',
				 5 => '4',
				 6 => '5',
				 7 => '6',
				 8 => '7',
				 9 => '8',
				10 => '9',
				11 => '0',
				_ => '\0',
			};
			if ccode != '\0'
			{
				write_screen(0, 0, ccode as u8, Some(Black), None);
			}
		}
	}
}
