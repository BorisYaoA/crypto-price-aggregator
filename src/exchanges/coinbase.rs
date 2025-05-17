use crate::model::PriceResult;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CoinbaseResponse {
    data: CoinbaseData,
}

#[derive(Debug, Deserialize)]
struct CoinbaseData {
    amount: String,
}

pub async fn fetch_coinbase(symbol: &str) -> Result<PriceResult> {
    let client = Client::new();

    let formatted_symbol = match symbol {
        "BTC/USD" => "BTC-USD",
        "ETH/USD" => "ETH-USD",
        _ => return Err(anyhow!("Unsupported symbol for Coinbase: {}", symbol)),
    };

    let url = format!("https://api.coinbase.com/v2/prices/{}/spot", formatted_symbol);
    let res = client.get(&url).send().await?.json::<CoinbaseResponse>().await?;

    let parsed_price = res.data.amount.parse::<f64>()?;

    Ok(PriceResult {
        exchange: "Coinbase".to_string(),
        symbol: "BTC/USD".to_string(),
        price: parsed_price,
    })
}