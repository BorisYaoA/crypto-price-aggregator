use crate::model::PriceResult;

pub fn print_prices(data: &[PriceResult]) {
    println!("{:<10} {:<10} {:<10}", "Exchange", "Symbol", "Price");
    for p in data {
        println!("{:<10} {:<10} {:.2}", p.exchange, p.symbol, p.price);
    }

    if !data.is_empty() {
        let avg: f64 = data.iter().map(|x| x.price).sum::<f64>() / data.len() as f64;
        println!("\nAverage price: {:.2}", avg);
    }
}