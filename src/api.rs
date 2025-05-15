use axum::{routing::get, Router};
use axum::extract::Query;
use serde::Deserialize;
use std::net::SocketAddr;
use crate::fetch::fetch_all_prices;
use crate::model::PriceResult;

#[derive(Deserialize)]
pub struct PriceQuery {
    symbol: String,
}

async fn price_handler(Query(q): Query<PriceQuery>) -> axum::Json<Vec<PriceResult>> {
    let data = fetch_all_prices(&q.symbol).await.unwrap_or_default();
    axum::Json(data)
}

pub async fn start_server() -> anyhow::Result<()> {
    let app = Router::new().route("/prices", get(price_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}