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

fn get(config: conf::Configuration) -> io::Result<()> {
    let username_child = Command::new("op")
        .arg("read")
        .arg(format!("op://{}/{}/username", config.vault, config.id))
        .output()?;

    if !username_child.status.success() {
        return io::Result::Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Unable to get username".to_string(),
        ));
    }

    let password_child = Command::new("op")
        .arg("read")
        .arg(format!("op://{}/{}/password", config.vault, config.id))
        .output()?;

    if !password_child.status.success() {
        return io::Result::Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Unable to get password".to_string(),
        ));
    }

    io::stdout().write_all(b"username=")?;
    io::stdout().write_all(&username_child.stdout)?;
    io::stdout().write_all(b"password=")?;
    io::stdout().write_all(&password_child.stdout)?;

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
