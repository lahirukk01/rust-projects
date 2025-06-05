use std::{env, io::ErrorKind, process};

use minigrep::{Config, run};

fn main() {
    // dbg!(args);

    let mut args = env::args().skip(1);

    let config = Config::build(&mut args).unwrap_or_else(|error| {
        eprintln!("Problem parsing args: {error}");
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Error: File not found: {}", config.file_path());
            }
            _ => {
                eprintln!("Application error: {}", e);
            }
        }
        process::exit(1);
    }

    // dbg!(config);
}
