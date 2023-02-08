use window_titles::{Connection, ConnectionTrait};

fn main() {
	let connection = Connection::new().unwrap();
	connection.window_titles().unwrap().into_iter()
		.for_each(|(id, title)| println!("FINAL RESULT:\n id: {:?}, title: {:?}", id, title));

		println!("test");
}
