use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, thread, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Bitcoin {
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ethereum {
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SP500 {
    price: f64,
}

trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64) -> Result<(), String>;
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => {
                let body: String = response.into_string().unwrap();
                let body_json: serde_json::Value = serde_json::from_str(&body).unwrap();
                let price = body_json["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
                Ok(price)
            }
            Err(e) => Err(format!("Error fetching Bitcoin price: {}", e)),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("bitcoin_price.txt")
            .map_err(|e| format!("Error creating file: {}", e))?;
        writeln!(file, "Bitcoin price: ${}", price)
            .map_err(|e| format!("Error writing to file: {}", e))?;
        Ok(())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => {
                let body: String = response.into_string().unwrap();
                let body_json: serde_json::Value = serde_json::from_str(&body).unwrap();
                let price = body_json["ethereum"]["usd"].as_f64().unwrap_or(0.0);
                Ok(price)
            }
            Err(e) => Err(format!("Error fetching Ethereum price: {}", e)),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("ethereum_price.txt")
            .map_err(|e| format!("Error creating file: {}", e))?;
        writeln!(file, "Ethereum price: ${}", price)
            .map_err(|e| format!("Error writing to file: {}", e))?;
        Ok(())
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=SPY&apikey=KPXE1N8FF9ECWD3Y";
        match ureq::get(url).call() {
            Ok(response) => {
                let body: String = response.into_string().unwrap();
                let body_json: serde_json::Value = serde_json::from_str(&body).unwrap();
                let price = body_json["Time Series (Daily)"]["2022-12-01"]["4. close"]
                    .as_f64()
                    .unwrap_or(0.0);
                Ok(price)
            }
            Err(e) => Err(format!("Error fetching S&P 500 price: {}", e)),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::create("sp500_price.txt")
            .map_err(|e| format!("Error creating file: {}", e))?;
        writeln!(file, "S&P 500 price: ${}", price)
            .map_err(|e| format!("Error writing to file: {}", e))?;
        Ok(())
    }
}

fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(SP500 { price: 0.0 }),
    ];

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price: ${}", price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("{}", e);
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}
