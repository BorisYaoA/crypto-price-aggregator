use crate::model::PriceResult;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BinanceResponse {
    symbol: String,
    price: String,
}

pub async fn fetch_binance(symbol: &str) -> Result<PriceResult> {
    let client = Client::new();

    let binance_symbol = match symbol {
        "BTC/USD" => "BTCUSDT",
        "ETH/USD" => "ETHUSDT",
        _ => return Err(anyhow!("Unsupported symbol for Binance: {}", symbol)),
    };

    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", binance_symbol);
    let res = client.get(&url).send().await?.json::<BinanceResponse>().await?;

    let parsed_price = res.price.parse::<f64>()?;

    Ok(PriceResult {
        exchange: "Binance".to_string(),
        symbol: symbol.to_string(),
        price: parsed_price,
    })
}
