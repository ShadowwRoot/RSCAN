pub struct Config {
    pub ip: String,
    pub start_port: u16,
    pub end_port: u16,
    pub verbose: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Usage: <ip> <start_port> <end_port> [--verbose]");
        }

        let ip = args[1].clone();
        let start_port = args[2].parse().map_err(|_| "Invalid start port")?;
        let end_port = args[3].parse().map_err(|_| "Invalid end port")?;
        let verbose = args.contains(&"--verbose".to_string());

        Ok(Config {
            ip,
            start_port,
            end_port,
            verbose,
        })
    }
}
