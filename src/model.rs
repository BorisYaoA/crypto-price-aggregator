use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct PriceResult {
    pub exchange: String,
    pub symbol: String,
    pub price: f64,
}