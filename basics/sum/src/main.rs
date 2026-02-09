use std::io;

fn main() {
	let mut num = String::new();
	let mut sum = 0;
	println!("Enter the number: ");
	io::stdin()
		.read_line(&mut num)
		.expect("[+]Please input a number");
	let mut num: u32 = num.trim().parse().expect("[+]Io Error");
	println!("The entered value: {num}");
	
	loop {
		if num == 0 {
			break;
		}
		let rem: u32 = num % 10;
		sum += rem;
		num /= 10;
	}
	println!("The sum of the digits of the given number: {}", sum);
}
