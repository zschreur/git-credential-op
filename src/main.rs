use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser, Debug)]
struct Opt {
    #[arg(short, long)]
    vault: String,
    #[arg(short, long)]
    id: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get,
    Store,
    Erase,
}

fn get(vault: String, id: String) -> io::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }
    }

    let username_child = Command::new("op")
        .arg("read")
        .arg(format!("op://{}/{}/username", vault, id))
        .output()?;

    let password_child = Command::new("op")
        .arg("read")
        .arg(format!("op://{}/{}/password", vault, id))
        .output()?;

    io::stdout().write_all(b"username=")?;
    io::stdout().write_all(&username_child.stdout)?;
    io::stdout().write_all(b"password=")?;
    io::stdout().write_all(&password_child.stdout)?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let opt = Opt::parse();

    match opt.cmd {
        Commands::Get => get(opt.vault, opt.id),
        _ => Ok(()),
    }
}
