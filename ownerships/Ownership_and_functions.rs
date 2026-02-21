fn main() {
	let s = String::from("Hello");
	takes_ownership(s.clone());
	println!("{s}");
	let x = 5;
	make_copy(x);
	println!("{x}");
}

fn takes_ownership(some_string: String) {
	println!("{some_string}");
}

fn make_copy(some_integer: i32) {
	println!("{some_integer}");
}
