use std::io;

fn main() {
	let mut a = String::new();
	let mut b = String::new();
	let mut c = String::new();
	println!("Enter the coefficent a: ");
	io::stdin()
		.read_line(&mut a)
		.expect("[+]IO Error.");

	println!("Enter the coefficent b: ");
        io::stdin()
                .read_line(&mut b)
                .expect("[+]IO Error.");

	println!("Enter the coefficent c: ");
        io::stdin()
                .read_line(&mut c)
                .expect("[+]IO Error.");
	
	let a: f32 = a.trim().parse().expect("[-] Please input a number: ");
	let b: f32 = b.trim().parse().expect("[-] Please input a number: ");
	let c: f32 = c.trim().parse().expect("[-] Please input a number: ");

	let d: f32 = (b * b) - (4.0 * a * c);
	println!("b^2 - 4ac = {}", d);
	
	if d == 0.0 {
		println!("[+] Since d = 0, there is only one root");
		let result = -b / (2.0 * a);
		println!("roots are: {}", result);
	} else if d > 0.0 {
		println!("[+] since d > 0, there are 2 roots");
		let result1 = (-b + d.sqrt()) / (2.0 * a);
		let result2 = (-b - d.sqrt()) / (2.0 * a);
		println!("roots are: {}, {}", result1, result2); 
	} else {
		println!("[+]Since d < 0, roots are imaginary");
	}

}
