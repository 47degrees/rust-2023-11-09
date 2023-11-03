struct Wrapped<T>(
	T // Owner of some `T`
);

enum Either<A, B> {
	A(A), // Owner of some `A`
	B(B), // Owner of some `B`
}

fn main() {
	let i = 5;            // Owner of `5`
	let w = Wrapped(i);   // Owner of `Wrapped(5)`; `i` still lives
	let a = Either::A(w); // Owner of `Either::A`; `w` destroyed by move
	let _x = f(a);        // Owner of some `Either`; `a` destroyed by move
} // `_x` destroyed by end of block

fn f<A>(
	e: Either<Wrapped<A>, Wrapped<A>> // Owner of some `Either`
) -> Either<Wrapped<A>, Wrapped<A>> { // Returns ownership of some `Either`
	match e {
		Either::A(a) => // `a` is owner of `Wrapped`; `e` destroyed by move
			Either::B(a),           // Temporary is owner of `Either::B`
		Either::B(b) => // `a` is owner of `Wrapped`; `e` destroyed by move
			Either::A(b)            // Temporary is owner of `Either::A`
	} // Return ownership of some `Either`
}
