use crate::exchanges::{binance::fetch_binance, coinbase::fetch_coinbase, kraken::fetch_kraken};
use crate::model::PriceResult;

pub async fn fetch_all_prices(symbol: &str) -> anyhow::Result<Vec<PriceResult>> {
    let (binance, coinbase, kraken) = tokio::join!(
        fetch_binance(symbol),
        fetch_coinbase(symbol),
        fetch_kraken(symbol)
    );

    Ok(vec![
        binance.unwrap_or_default(),
        coinbase.unwrap_or_default(),
        kraken.unwrap_or_default(),
    ])
}