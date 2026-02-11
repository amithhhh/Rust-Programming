fn main() {
	let num:u32 = 5;
	println!("The factorial of the given number is: {}", fact(num));
}

fn fact(num:u32) -> u32 {
	if num == 0 {
		1
	} else if num == 1 {
		1
	} else {
		num * fact(num - 1)
	}
} 
