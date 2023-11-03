use std::rc::Rc;
use std::thread::spawn;

fn main() {
	let shared: Rc<i32> = Rc::new(10);
	let copy = Rc::clone(&shared);
	let forked = spawn(move ||
		{
			println!("first thread: {}", *copy);
		}
	);
	forked.join().unwrap();
	println!("main thread: {}", *shared);
}
