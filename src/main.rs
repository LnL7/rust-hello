use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn parse_line(line: String) -> Option<String> {
    if line.starts_with("NAME=") {
        Some(line[5..].into())
    } else {
        None
    }
}

fn os_name() -> io::Result<String> {
    let os_release = File::open("/etc/os-release")?;
    for line in BufReader::new(os_release).lines() {
        if let Some(name) = parse_line(line?) {
            return Ok(name);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "OS name not detected"))
}

fn main() {
    let greeting = match os_name() {
        Ok(name) => name,
        Err(_) => "world".into(),
    };
    println!("Hello, {}!", greeting);
}
