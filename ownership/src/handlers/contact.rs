use crate::args::{ContactCommand, ContactSubcommand};

use sqlite::Connection;

pub fn handle_contact_command(conn: Connection, c: ContactCommand) {
    match c.command {
        ContactSubcommand::Create(c) => {
            println!("Contact create for {} {} !", c.first_name, c.last_name);
            let create_contact_query = format!(
                "INSERT INTO contacts (first_name, last_name, phone_number) VALUES ('{}','{}' ,'{}')",
                c.first_name, c.last_name, c.phone_number
            );
            match conn.execute(create_contact_query) {
                Ok(exec) => exec,
                Err(e) => print!("Error {}", e),
            };
        }
    }
}
