fn main() {
	let mut s = String::from("Hello, World");
	change(&mut s);
	println!("{s}");
}

fn change(some_str: &mut String) {
	some_str.push_str("!");
}
