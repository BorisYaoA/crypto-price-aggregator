use crate::model::PriceResult;
use reqwest::Client;

pub async fn fetch_binance(symbol: &str) -> anyhow::Result<PriceResult> {
    let pair = symbol.replace("/", "").to_uppercase(); // e.g., BTC/USD -> BTCUSD
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", pair);

    let client = Client::new();
    let resp = client.get(&url).send().await?.json::<serde_json::Value>().await?;
    let price = resp["price"].as_str().unwrap().parse::<f64>()?;

    Ok(PriceResult {
        exchange: "Binance".into(),
        symbol: symbol.into(),
        price,
    })
}