# Crypto Price Aggregator

Aggregates live BTC/USD prices from multiple crypto exchanges using async Rust.

## Features

- CLI or REST API modes
- Fetches prices from Binance, Kraken, and Coinbase
- Uses async HTTP (reqwest + tokio)
- Easy to extend (add new exchanges)
- Lightweight and production-ready

## Usage

```bash
cargo run -- --symbol BTC/USD        # CLI mode
cargo run -- --rest                  # Start REST API on localhost:3000 and curl http://127.0.0.1:3000/prices
