fn owner_dropped_while_borrowed() {
	let outer_borrow: &i32;
	{
		let inner_borrow: &i32;
		let owner: i32 = 10;
		inner_borrow = &owner;
		outer_borrow = inner_borrow;
	}
	println!("heh, heh = {}", outer_borrow);
}

fn main() {
	// Don't need to do anything but exist, to get the compiler to
	// complain about other things.
}
