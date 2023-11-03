use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{JoinHandle, spawn};

fn main() {
	let shared: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
	let join_handles: Vec<JoinHandle<()>> = (1..=10)
		.map(|_| { // Don't care about the subscript, only running 10 times
			let copy = Arc::clone(&shared);
			spawn(move || {
				let mut guarded_value: MutexGuard<i32> = copy.lock().unwrap();
				*guarded_value += 1;
			})
		})
		.collect(); // Iterators are lazy, so must collect
	// Wait for all threads to complete.
	join_handles.into_iter().for_each(|h| h.join().unwrap());
	println!("{}", *shared.lock().unwrap()); // Prints "10\n"
}
