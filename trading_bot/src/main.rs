use std::error::Error;
use reqwest::Error as ReqwestError;
use serde_json::Value; // Add this line

async fn fetch_klines() -> Result<(), ReqwestError> {
    let binance_api_url = "https://api.binance.com/api/v3/klines";
    let symbol = "BTCUSDT";
    let interval = "1m";
    let limit = "1"; // Number of klines to retrieve
    
    let params = [
        ("symbol", symbol),
        ("interval", interval),
        ("limit", limit)
    ];

    let client = reqwest::Client::new();
    let response = client.get(binance_api_url)
        .query(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let klines: Vec<Vec<Value>> = response.json().await?; // Modify this line
        println!("Last klines for {}: {:?}", symbol, klines);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fetch_klines().await?;
    Ok(())
}
