use std::io;

fn main() {
	let mut age = String::new();
	io::stdin()
		.read_line(&mut age)
		.expect("[-] IO Error");

	let age: i32 = age.trim().parse().expect("[-] Not a number.");
	
	if age > 18 {
		println!("[+] You are eligible.");
	} else {
		println!("[-] You are not eligible.")
	}
}
