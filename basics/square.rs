fn main() {
	let num = 10;

	match num > 5 {
		true => {
			println!("{}", num * num);
		}
		false => {
			println!("[-]Sorry value less than 5");
		}
	}
}
