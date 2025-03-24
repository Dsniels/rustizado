mod shapes;
use shapes::{area::Area, collisions::Colliadable, rectangle::Rectangle};

fn main() {
	let rec = Rectangle::default();
	let rec1 = Rectangle::default();

	let collide = rec.collide(&rec1);


	println!("{}", collide);
}
