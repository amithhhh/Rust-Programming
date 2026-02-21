fn main() {
	let mut s = String::from("Hello");
	{
		let _r1 = &mut s;
		println!("{_r1}");
	}
	let _r2 = &mut s;

	println!("{_r2}");
}
