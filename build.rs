use clap_complete::aot::Zsh;
use clap_complete::generate_to;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let mut cmd = build_command();
    let path = generate_to(Zsh, &mut cmd, "nb", &outdir)?;
    println!("cargo:warning=completion file is generated: {path:?}");
    Ok(())
}
