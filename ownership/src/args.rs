use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct ContactAppArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// create, update, delete, or show users
    Contact(ContactCommand),
}

#[derive(Debug, Args)]
pub struct ContactCommand {
    #[clap(subcommand)]
    pub command: ContactSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ContactSubcommand {
    Create(CreateContact),
    //Update(UpdateContact),
    //Delete(DeleteContact),
    //Show,
}

#[derive(Debug, Args)]
pub struct CreateContact {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}
