use crate::exchanges::{binance::fetch_binance, coinbase::fetch_coinbase, kraken::fetch_kraken};
use crate::model::PriceResult;
use futures::future;

pub async fn fetch_all_prices(symbol: &str) -> anyhow::Result<Vec<PriceResult>> {
    let tasks = vec![
        fetch_binance(symbol),
        fetch_coinbase(symbol),
        fetch_kraken(symbol),
    ];

    let results = future::join_all(tasks).await;
    Ok(results.into_iter().filter_map(Result::ok).collect())
}