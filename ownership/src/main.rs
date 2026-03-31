use clap::Parser;

#[derive(Parser)]
enum Cli {
    AddContact(ContactInfo),
}
#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct ContactInfo {
    #[arg(long)]
    #[arg(short)]
    first_name: String,
    #[arg(long)]
    #[arg(short)]
    last_name: String,
    #[arg(long)]
    #[arg(short)]
    phone_number: String,
}
struct ContactStore {
    store: Vec<ContactInfo>,
}
impl ContactStore {
    fn store_contact(&mut self, c: ContactInfo) {
        self.store.push(c)
    }
    fn list_contact(&self) {
        for item in &self.store {
            println!("First Name: {}", item.first_name);
        }
    }
}

fn main() {
    let mut store = ContactStore { store: vec![] };
    match Cli::parse() {
        Cli::AddContact(args) => {
            let first_name = args.first_name;
            let last_name = args.last_name;
            let phone_number = args.phone_number;
            println!(
                "ADDED: \n| First name: {first_name} \n| Last name: {last_name} \n| Phone Number: {phone_number}"
            );
            store.store_contact(ContactInfo {
                first_name,
                last_name,
                phone_number,
            });
        }
    }
}
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
