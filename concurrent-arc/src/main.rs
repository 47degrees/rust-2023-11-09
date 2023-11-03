use std::sync::Arc;
use std::thread::spawn;

fn main() {
	let shared: Arc<i32> = Arc::new(10);
	let copy = Arc::clone(&shared);
	let forked = spawn(move || {
		println!("first thread: {}", *copy);
	});
	forked.join().unwrap();
	println!("main thread: {}", *shared);
}
