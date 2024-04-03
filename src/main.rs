use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::Command;

mod conf;

#[derive(Parser, Debug)]
struct Opt {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get,
    Store,
    Erase,
}

fn read_field(config: &conf::Configuration, field: &str) -> io::Result<Vec<u8>> {
    let output = Command::new("op")
        .arg("read")
        .arg(format!("op://{}/{}/{}", config.vault, config.id, field))
        .output()?;

    if output.status.success() {
        io::Result::Ok(output.stdout)
    } else {
        io::Result::Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Unable to get {}", field),
        ))
    }
}

fn get(config: conf::Configuration) -> io::Result<()> {
    let username = read_field(&config, "username")?;
    let password = read_field(&config, "password")?;

    io::stdout().write_all(b"username=")?;
    io::stdout().write_all(&username)?;
    io::stdout().write_all(b"password=")?;
    io::stdout().write_all(&password)?;

    Ok(())
}

#[derive(Debug)]
enum OPGitError {
    ConfigurationError(config::ConfigError),
    IOError(io::Error),
}

fn main() -> Result<(), OPGitError> {
    let opt = Opt::parse();
    let c = conf::load_configuration().map_err(OPGitError::ConfigurationError)?;

    match opt.cmd {
        Commands::Get => get(c).map_err(OPGitError::IOError),
        _ => Ok(()),
    }
}
