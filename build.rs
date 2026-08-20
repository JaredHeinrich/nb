use clap::CommandFactory;
use clap_complete::aot::Zsh;
use clap_complete::generate_to;
use std::env;
use anyhow::{Result, anyhow};

include!("src/cli.rs");

fn main() -> Result<()> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Err(anyhow!("Environment Variable `OUT_DIR` not found"));
    };
    let Some(project_root) = env::var_os("CARGO_MANIFEST_DIR") else {
        return Err(anyhow!("Environment Variable `CARGO_MANIFEST_DIR` not found"));
    };

    let mut cmd = Cli::command();
    let path = generate_to(Zsh, &mut cmd, "rn", &outdir)?;
    let relative_path = path.strip_prefix(project_root)?;
    println!(
        "cargo:warning=zsh completion file is generated: {}",
        relative_path.display()
    );
    Ok(())
}
