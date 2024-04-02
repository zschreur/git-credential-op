use std::env;

enum Operation {
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
    let args: Vec<String> = env::args().collect();
    let op = match args.last().map(|s| s.as_str()) {
        Some("get") => Operation::Get,
        Some("store") => Operation::Store,
        Some("erase") => Operation::Erase,
        _ => panic!("Invalid operation: {}", args[1]),
    };

    match op {
        Operation::Get => get(),
        _ => Ok(()),
    }
}
