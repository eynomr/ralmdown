use rusqlite::{Connection};

#[derive(Debug)]
struct Thing {
    definitions: String
}
fn main() {
    let path = "./db/things.db";
    let db = Connection::open(path).unwrap();

    let mut stmt = db.prepare("SELECT definition FROM things").unwrap();

    let thing = stmt.query_map([], |row| {
        Ok(Thing {
            definitions: row.get(0).unwrap()
        })
    }).unwrap();

    for t in thing {
        println!("{:?}", t.unwrap())
    }

}