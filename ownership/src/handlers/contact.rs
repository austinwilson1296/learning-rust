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
        ContactSubcommand::Show => {
            println!("Printing contacts from CONTACTY!\n");
            let find_all_contacts = "SELECT * FROM contacts";
            for row in conn
                .prepare(find_all_contacts)
                .unwrap()
                .into_iter()
                .map(|row| row.unwrap())
            {
                println!("---------------------");
                let first_name = row.read::<&str, _>("first_name");
                let last_name = row.read::<&str, _>("last_name");
                let phone_number = row.read::<&str, _>("phone_number");
                println!(
                    "First Name: {}\nLast Name {}\nPhone Number {}\n",
                    first_name, last_name, phone_number
                );
                println!("---------------------");
            }
        }
    }
}
