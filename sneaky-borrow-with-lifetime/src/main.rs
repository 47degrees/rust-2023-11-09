fn annotated_assign_through_borrow<
	'a: 'b,
	'b
>(
	a: &mut &'a i32,
	b: &mut &'b i32,
) {
	*b = *a;
}

fn owner_dropped_while_borrowed() {
	let outer_owner = 10;
	let mut outer_borrow = &outer_owner;
	{
		let inner_owner = 10;
		let mut inner_borrow = &inner_owner;
		annotated_assign_through_borrow(
			&mut inner_borrow,
			&mut outer_borrow
		);
	}
	println!("heh, heh = {}", outer_borrow);
}

fn main() {
	println!("Hello, world!");
}
