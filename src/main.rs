use anyhow::Result;
use app::App;

use crate::message::Message;

mod app;
mod config;
mod error;
mod file_operations;
mod message;

fn main() {
    let app = App::new(config::Config::load());

    let command = app.get_command();
    let matches = command.get_matches();

    let result = app.handle_command(matches);
    print_result(result);
}

fn print_result(result: Result<Message>) {
    match result {
        Ok(m) => print!("{m}"),
        Err(e) => print!("{e}"),
    }
}
