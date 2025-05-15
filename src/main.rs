use clap::Parser;
use crypto_price_aggregator::{fetch::fetch_all_prices, utils::print_prices};

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
        crypto_price_aggregator::api::start_server().await?;
    } else {
        let prices = fetch_all_prices(&args.symbol).await?;
        print_prices(&prices);
    }

    Ok(())
}
