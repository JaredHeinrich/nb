use clap::CommandFactory;
use clap_complete::aot::Zsh;
use clap_complete::generate_to;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let mut cmd = Cli::command();
    let path = generate_to(Zsh, &mut cmd, "rn", &outdir)?;
    println!(
        "cargo:warning=completion file is generated: {}",
        path.display()
    );
    Ok(())
}
