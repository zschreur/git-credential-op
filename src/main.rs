use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Opt {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get,
    Store,
    Erase,
}

fn get() -> std::io::Result<()> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::parse();

    match opt.cmd {
        Commands::Get => get(),
        _ => Ok(()),
    }
}
