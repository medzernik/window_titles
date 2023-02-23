use window_titles::{Connection, ConnectionTrait};

use itertools::Itertools;

fn main() {
    let connection = Connection::new().unwrap();
    connection
        .window_titles()
        .unwrap()
        .iter()
        .into_iter()
        .unique_by(|(id, title)| title)
        .for_each(|(id, title)| println!("FINAL RESULT:\n id: {:?}, title: {:?}", id, title));

    println!("test");
}
