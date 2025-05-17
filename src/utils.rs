use crate::model::PriceResult;

pub fn print_prices(prices: &[PriceResult]) {
    for price in prices {
        println!(
            "{} price on {}: ${}",
            price.symbol, price.exchange, price.price
        );
    }
}