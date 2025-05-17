use clap::Parser;

mod api;
mod fetch;
mod model;
mod utils;
mod exchanges;

use fetch::fetch_all_prices;
use utils::print_prices;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "BTC/USD")]
    symbol: String,
    #[arg(short, long)]
    rest: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.rest {
        api::start_server().await?;
    } else {
        let prices = fetch_all_prices(&args.symbol).await?;
        print_prices(&prices);
    }

    Ok(())
}