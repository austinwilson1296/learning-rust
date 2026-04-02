use crate::args::{ContactCommand, ContactSubcommand};

pub fn handle_contact_command(c: ContactCommand) {
    match c.command {
        ContactSubcommand::Create(c) => {
            println!("Contact create for {} {} !", c.first_name, c.last_name);
        }
    }
}
