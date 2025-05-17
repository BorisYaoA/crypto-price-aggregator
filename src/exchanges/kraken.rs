use crate::model::PriceResult;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KrakenResponse {
    error: Vec<String>,
    result: std::collections::HashMap<String, KrakenTicker>,
}

#[derive(Debug, Deserialize)]
struct KrakenTicker {
    c: Vec<String>, // c = [ <price>, <lot volume> ]
}

pub async fn fetch_kraken(symbol: &str) -> Result<PriceResult> {
    let client = Client::new();

    // Kraken uses different pair names than standard format, e.g., "BTC/USD" => "XBTUSD"
    let kraken_symbol = match symbol {
        "BTC/USD" => "XXBTZUSD",
        "ETH/USD" => "XETHZUSD",
        _ => return Err(anyhow!("Unsupported symbol for Kraken: {}", symbol)),
    };

    let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_symbol);
    let res = client.get(&url).send().await?.json::<KrakenResponse>().await?;

    if !res.error.is_empty() {
        return Err(anyhow!("Kraken API error: {:?}", res.error));
    }

    let ticker = res.result.get(kraken_symbol).ok_or_else(|| anyhow!("Symbol not found in Kraken response"))?;
    let price = ticker.c.get(0).ok_or_else(|| anyhow!("No current price found"))?;
    let parsed_price = price.parse::<f64>()?;

    Ok(PriceResult {
        exchange: "Kraken".to_string(),
        symbol: "BTC/USD".to_string(),
        price: parsed_price,
    })
}