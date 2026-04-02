use sqlite::Connection;

pub fn db_setup() -> Connection {
    let connection = sqlite::open("./contacts.db").unwrap();
    let create_tables =
        "CREATE TABLE IF NOT EXISTS contacts (first_name TEXT, last_name TEXT,phone_number TEXT)";
    connection.execute(create_tables).unwrap();
    connection
}
