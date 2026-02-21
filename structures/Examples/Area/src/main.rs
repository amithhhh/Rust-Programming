struct Rectangle {
	width: u32,
	height: u32
}

struct Triangle {
	width: u32,
	height: u32
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};
	let triangle = Triangle {
		width: 45,
		height: 92,
	};
	println!("The area of the rectangle is {} square pixels.", area(&rect1));
	println!("The area of the triangle is {} square pixels.", tri_area(&triangle));
}

fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

fn tri_area(tri: &Triangle) -> u32 {
	(tri.width * tri.height) / 2
}
