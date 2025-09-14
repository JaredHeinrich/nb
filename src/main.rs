use app::App;
use clap::Command;
use crate::message::Message;

mod app;
mod config;
mod message;
mod error;
mod file_operations;

fn main() {
    let app = App::new(config::Config::load());
    let nb_command = Command::new("nb")
        .version("0.0.1")
        .about("CLI note book (nb)")
        .subcommand(Command::new("list"))
        .subcommand(Command::new("init"))
        .subcommand(Command::new("status"))
        .subcommand(Command::new("show"))
        .subcommand(Command::new("create"))
        .subcommand(Command::new("add"))
        .subcommand(Command::new("archive"))
        .subcommand_required(true)
        .get_matches();


    let return_message = match nb_command.subcommand() {
        //Some(("list", _sub_matches)) => app.get_notebooks(),
        None => Message::MissingSubcommand,
        _ => unreachable!("unknown error"),
    };
}
