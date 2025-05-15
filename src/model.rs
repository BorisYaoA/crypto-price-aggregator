use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct PriceResult {
    pub exchange: String,
    pub symbol: String,
    pub price: f64,
}