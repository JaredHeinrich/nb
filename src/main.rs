use anyhow::Result;
use app::App;

use crate::{file_operations::FileSystem, message::Message};

mod app;
mod cli;
mod config;
mod error;
mod file_operations;
mod message;
mod utils;
#[cfg(test)]
mod mock_fs;

fn main() {
    clap_complete::CompleteEnv::with_factory(cli::build_command).complete();

    let config = config::Config::load();
    let fs = FileSystem;
    let mut app = App::new(config, fs);
    let command = app.build_command().unwrap(); // TODO remove unwrap
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
