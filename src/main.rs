use anyhow::Result;
use app::App;
use clap::Parser;

use crate::{cli::Cli, file_operations::FileSystem, message::Message};

mod app;
mod cli;
mod config;
mod error;
mod file_operations;
mod message;

#[cfg(test)]
mod mock_fs;

fn main() {
    let result = run();
    print_result(result);
}

fn run() -> Result<Message> {
    let fs = FileSystem;
    let config = config::Config::build(&fs)?;
    let mut app = App::new(config, fs)?;
    let command = Cli::parse();
    app.handle_command(command)
}

fn print_result(result: Result<Message>) {
    match result {
        Ok(m) => print!("{m}"),
        Err(e) => print!("{e}"),
    }
}
