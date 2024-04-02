use std::env;

enum Operation {
    Get,
    Store,
    Erase,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let op = match args.last().map(|s| s.as_str()) {
        Some("get") => Operation::Get,
        Some("store") => Operation::Store,
        Some("erase") => Operation::Erase,
        _ => panic!("Invalid operation: {}", args[1]),
    };

    match op {
        Operation::Get => (),
        _ => (),
    }
}
