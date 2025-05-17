use axum::{routing::get, Router};
use hyper::Server;
use std::net::SocketAddr;

use crate::fetch::fetch_all_prices;
use crate::model::PriceResult;

async fn get_prices() -> axum::Json<Vec<PriceResult>> {
    let prices = fetch_all_prices("BTC/USD").await.unwrap_or_default();
    axum::Json(prices)
}

pub async fn start_server() -> anyhow::Result<()> {
    let app = Router::new().route("/prices", get(get_prices));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}