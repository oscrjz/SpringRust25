use reqwest::blocking::Client;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use std::fs::OpenOptions;
use std::io::Write;

use num_cpus;

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn parse_args() -> (Vec<String>, usize, u64, u32) {
    let args: Vec<String> = env::args().collect();
    let mut urls: Vec<String> = Vec::new();
    let mut file_path = None;
    let mut workers = num_cpus::get();
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                if i + 1 < args.len() {
                    file_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--workers" => {
                if i + 1 < args.len() {
                    workers = args[i + 1].parse().unwrap_or(workers);
                    i += 1;
                }
            }
            "--timeout" => {
                if i + 1 < args.len() {
                    timeout_secs = args[i + 1].parse().unwrap_or(5);
                    i += 1;
                }
            }
            "--retries" => {
                if i + 1 < args.len() {
                    retries = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            s if !s.starts_with("--") => urls.push(s.to_string()),
            _ => {}
        }
        i += 1;
    }

    if let Some(path) = file_path {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(url) = line {
                    let trimmed = url.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        urls.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    (urls, workers, timeout_secs, retries)
}

fn check_website(
    url: String,
    client: &Client,
    timeout: Duration,
    retries: u32,
) -> WebsiteStatus {
    let mut attempts = 0;
    let start_time = Instant::now();
    let timestamp = SystemTime::now();

    let result = loop {
        attempts += 1;
        let response = client.get(&url).timeout(timeout).send();
        match response {
            Ok(resp) => break Ok(resp.status().as_u16()),
            Err(_e) if attempts <= retries => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => break Err(e.to_string()),
        }
    };

    let elapsed = start_time.elapsed();
    WebsiteStatus {
        url,
        action_status: result,
        response_time: elapsed,
        timestamp,
    }
}

fn write_json(results: &[WebsiteStatus]) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("status.json")
        .expect("Unable to open status.json");

    writeln!(file, "[").unwrap();
    for (i, status) in results.iter().enumerate() {
        let status_str = match &status.action_status {
            Ok(code) => format!("\"status\": {}", code),
            Err(e) => format!("\"error\": \"{}\"", e),
        };

        let json = format!(
            "  {{ \"url\": \"{}\", {}, \"time_ms\": {}, \"timestamp\": {:?} }}",
            status.url,
            status_str,
            status.response_time.as_millis(),
            status.timestamp
        );

        if i + 1 == results.len() {
            writeln!(file, "{}", json).unwrap();
        } else {
            writeln!(file, "{},", json).unwrap();
        }
    }
    writeln!(file, "]").unwrap();
}

fn main() {
    let (urls, workers, timeout_secs, retries) = parse_args();
    let client = Arc::new(Client::builder().build().unwrap());
    let timeout = Duration::from_secs(timeout_secs);

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); 

    let (result_tx, result_rx) = mpsc::channel();

    for _ in 0..workers {
        let rx = Arc::clone(&rx); 
        let result_tx = result_tx.clone();
        let client = Arc::clone(&client);
        thread::spawn(move || {
            while let Ok(url) = rx.lock().unwrap().recv() {
                let status = check_website(url, &client, timeout, retries);
                println!(
                    "{} - {} - {}ms",
                    status.url,
                    match &status.action_status {
                        Ok(code) => format!("HTTP {}", code),
                        Err(e) => format!("Error: {}", e),
                    },
                    status.response_time.as_millis()
                );
                result_tx.send(status).unwrap();
            }
        });
    }

    for url in &urls {
        tx.send(url.clone()).unwrap();
    }

    drop(tx);

    let mut results = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(status) = result_rx.recv() {
            results.push(status);
        }
    }

    write_json(&results);
}
