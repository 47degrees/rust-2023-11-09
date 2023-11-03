struct Wrapped<T>(
	T // Owner of some `T`
);

enum Either<'a, 'b, A, B> {
	A(&'a A), // Immutable borrow of some `A` with lifetime 'a
	B(&'b B), // Immutable borrow of some `B` with lifetime 'b
}

fn main() {
	let w = Wrapped(5);        // Owner of `Wrapped(5)`
	let mut a = Either::A(&w); // Mutable Owner of `Either`; `w` immutably borrowed
	f(&mut a);                 // `a` mutably borrowed, updated by call
} // `a` & `w` destroyed by end of block

fn f<'a, A>(
	e: &mut Either<'a, 'a, A, A> // Mutable borrow of some `Either`
) {
	*e = match e {
		Either::A(a) => // `a` owns borrow of `A`
			Either::B(a), // `e` becomes owner of new `Either`; `a` moved
		Either::B(b) => // `a` owns borrow of `A`
			Either::A(b)  // `e` becomes owner of new `Either`; `b` moved
	}
}
