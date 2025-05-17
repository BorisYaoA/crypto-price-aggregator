use serde::Serialize;

#[derive(Serialize)]
pub struct PriceResult {
    pub exchange: String,
    pub price: f64,
}