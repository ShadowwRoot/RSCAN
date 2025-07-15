use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use colored::*;
use rayon::prelude::*;
use crate::args::Config;
use std::io::{Read, Write};

use chrono::Local;  // ← ajout chrono pour timestamp

pub fn scan_ports(config: Config) {
    println!("Starting RSCAN...");

    let start_time = Instant::now();
    let ports: Vec<u16> = (config.start_port..=config.end_port).collect();
    let timeout = Duration::from_millis(500);
    let ip_list = expand_ips(&config.ip);

    let mut total_open = 0;
    let mut total_ports = 0;

    for ip in ip_list {
        println!("Scanning {}...", ip);

        if format!("{}:0", ip).to_socket_addrs().is_err() {
            eprintln!("❌ Failed to resolve domain: {}", ip);
            continue;
        }

        let open_ports: Vec<(u16, String)> = ports.par_iter()
            .filter_map(|&port| {
                let address = format!("{}:{}", ip, port);
                if let Ok(mut stream) = TcpStream::connect_timeout(&address.to_socket_addrs().unwrap().next().unwrap(), timeout) {
                    let mut banner = String::new();

                    if port == 80 || port == 8080 {
                        let _ = stream.write_all(b"GET / HTTP/1.0\r\n\r\n");
                    }

                    let mut buf = [0; 1024];
                    stream.set_read_timeout(Some(Duration::from_millis(1000))).ok();
                    if let Ok(size) = stream.read(&mut buf) {
                        banner = String::from_utf8_lossy(&buf[..size])
                            .chars()
                            .filter(|c| c.is_ascii_graphic() || c.is_whitespace())
                            .take(200)
                            .collect();
                    }

                    Some((port, banner))
                } else {
                    None
                }
            })
            .collect();

        total_open += open_ports.len();
        total_ports += ports.len();

        for port in &ports {
            if let Some((_, banner)) = open_ports.iter().find(|(p, _)| p == port) {
                println!("{}", format!("Port {} OPEN ({}) - Banner: {}", port, service_name(*port), banner).green().bold());
            } else if config.verbose {
                println!("{}", format!("Port {} CLOSED", port).red());
            }
        }

        let result_text = open_ports.iter()
            .map(|(p, b)| format!("Port {} ({}) - Banner: {}", p, service_name(*p), b))
            .collect::<Vec<String>>()
            .join("\n");

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("scan_results_{}_{}.txt", ip.replace(".", "_"), timestamp);

        std::fs::write(
            &filename,
            format!("IP: {}\n{}\n", ip, result_text)
        ).expect("Failed to write results to file");
        println!("Résultats exportés dans {}", filename);
    }

    let duration = start_time.elapsed();
    println!(
        "\nScan terminé: {} ports ouverts sur {} testés",
        total_open, total_ports
    );
    println!("Scan réalisé en {:.2?} secondes", duration);
}

fn service_name(port: u16) -> &'static str {
    match port {
        22 => "SSH",
        80 => "HTTP",
        443 => "HTTPS",
        21 => "FTP",
        25 => "SMTP",
        3306 => "MySQL",
        8080 => "HTTP-ALT",
        _ => "UNKNOWN",
    }
}

fn expand_ips(ip_input: &str) -> Vec<String> {
    if let Some((prefix, range)) = ip_input.rsplit_once('.') {
        if range.contains('-') {
            let parts: Vec<&str> = range.split('-').collect();
            let start: u8 = parts[0].parse().unwrap();
            let end: u8 = parts[1].parse().unwrap();
            (start..=end)
                .map(|i| format!("{}.{}", prefix, i))
                .collect()
        } else {
            vec![ip_input.to_string()]
        }
    } else {
        vec![ip_input.to_string()]
    }
}
