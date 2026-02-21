fn main() {
	//let s = "Hello"; //Immutable, stores in stack, read-only
	let mut w = String::from("Hello, "); //mutable, stores in heap
	w.push_str("World!");
	println!("{w}");

	let s1 = String::from("Hello, World");
	//let s2 = s1;
	println!("{s1},!");

	let s3 = String::from("Hello, World");
        let s4 = s3.clone();
        println!("{s3}, {s4}");
}
