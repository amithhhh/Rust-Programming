fn main() {
	let ref = dangle();
}

fn dangle() -> &String {
	let s = String::from("Hello");
	&s
}
