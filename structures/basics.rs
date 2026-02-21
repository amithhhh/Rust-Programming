#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	let mut user1 = User {
		active: true,
		username: String::from("kannan"),
		email: String::from("kannan@gmail.com"),
		sign_in_count: 1,
	};
	user1.email = String::from("kpkannan@gmail.com");

	let user2 = build_user("amithajith7025@gmail.com".to_string(), "amithhhhh".to_string());
	let user3 = User {
		email: String::from("sumal@mail.com"),
		..user1
	};
	
	println!("{:?}", user1);
	println!("{:#?}", user2);
	println!("{:#?}", user3);
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username: username,
		email: email,
		sign_in_count: 1,
	}
}
