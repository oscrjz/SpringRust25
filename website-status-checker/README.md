# Website Status Checker

A multi-threaded Rust tool that checks the status of websites by sending HTTP GET requests.

## Features

- Accepts URLs from a file or command line
- Configurable thread count, timeout, and retry count
- Outputs results in both terminal and JSON file (`status.json`)
- Gracefully handles unreachable sites

## Usage

### Build

```bash
cargo build --release

cargo run -- --file sites.txt --workers 4 --timeout 10 --retries 3

# Check from file with 4 threads and 3 second timeout, 1 retry
cargu run --file sites.txt --workers 4 --timeout 3 --retries 1

# Check URLs directly
cargo run https://google.com https://example.com --timeout 2
