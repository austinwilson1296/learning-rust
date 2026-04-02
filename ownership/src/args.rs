use clap::{Args, Parser, Subcommand};

use clap::builder::styling::{AnsiColor, Effects, Styles};

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Debug, Parser)]
#[command(styles = get_styles())]
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
