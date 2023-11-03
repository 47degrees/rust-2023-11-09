fn assign_through_borrow(
	a: &mut &i32,
	b: &mut &i32
) {
	*b = *a;
}

fn owner_dropped_while_borrowed() {
	let outer_owner = 10;
	let mut outer_borrow = &outer_owner;
	{
		let inner_owner = 10;
		let mut inner_borrow = &inner_owner;
		assign_through_borrow(
			&mut inner_borrow,
			&mut outer_borrow
		);
	}
	println!("heh, heh = {}", outer_borrow);
}

fn main() {
	println!("Hello, world!");
}
