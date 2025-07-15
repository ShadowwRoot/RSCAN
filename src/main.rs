mod args;
mod scanner;

use args::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("❌ Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    scanner::scan_ports(config);
}
