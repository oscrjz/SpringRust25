use std::fs::File;
use std::io::{self, BufRead};

struct Config {
    name: String,
    sid: String,
}

impl Config {
    fn from_file(path: &str) -> io::Result<Config> {
        let file = File::open(path)?;
        let mut lines = io::BufReader::new(file).lines();

        let name = lines.next().unwrap()?.trim().to_string();
        let sid = lines.next().unwrap()?.trim().to_string();

        Ok(Config { name, sid })
    }
}

fn reading_from_file() {
    match Config::from_file("config.txt") {
        Ok(config) => {
            println!("Name: {}", config.name);
            println!("SID: {}", config.sid);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn main() {
    reading_from_file();
}
