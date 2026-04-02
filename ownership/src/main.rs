mod args;
mod db;
mod handlers;
use crate::args::EntityType;
use crate::db::db_setup;
use crate::handlers::contact::handle_contact_command;
use args::ContactAppArgs;
use clap::Parser;

fn main() {
    let conn = db_setup();
    let args = ContactAppArgs::parse();

    match args.entity_type {
        EntityType::Contact(contact) => handle_contact_command(conn, contact),
    }
}
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped
