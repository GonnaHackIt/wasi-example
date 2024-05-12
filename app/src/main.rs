#[allow(warnings)]
mod bindings;

use bindings::docs::validator::form::validate;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "validator")]
struct Command {
    email: String,
    phone: String,
}

impl Command {
    fn run(self) {
        let validated = validate(&self.email, &self.phone);

        let msg = if validated { "Good form" } else { "Bad form" };

        println!("{msg}");
    }
}

fn main() {
    Command::parse().run()
}
