# Demo Oracle Home Test

## Project Overview
This project listens to live Bitcoin-to-USD (BTC/USD) price streams from multiple WebSocket connections across different cryptocurrency exchanges. It collects price data, verifies cryptographic signatures to ensure data integrity, and calculates the average price of Bitcoin in USD. 

The project stores the aggregated results and individual exchange results in separate files for further analysis. 

## Features
- **Multiple WebSocket Connections**: Connects to multiple cryptocurrency exchanges (like Binance, Kraken, etc.) to fetch live BTC/USD prices.
- **Cryptographic Signature Verification**: Uses cryptographic verification to ensure the integrity of the data.
- **Concurrent Data Fetching**: Efficiently handles multiple WebSocket streams using Rust's async runtime.
- **Price Aggregation**: Calculates the average BTC/USD price from all streams and stores the result.
- **Benchmarking**: Performance testing using Criterion.

## Dependencies
The project relies on the following key dependencies:
- **[tokio](https://crates.io/crates/tokio)** and **[tokio-tungstenite](https://crates.io/crates/tokio-tungstenite)**: Asynchronous runtime and WebSocket library for handling concurrent connections.
- **[k256](https://crates.io/crates/k256)**: Library for elliptic curve signature verification (used for verifying the data signatures).
- **[criterion](https://crates.io/crates/criterion)**: Benchmarking tool for performance testing.

## Project Structure
```
.
├── benches
│   └── bench_test.rs             # Benchmarking tests
├── btcusd_average.txt            # Aggregated BTC/USD price results
├── Cargo.lock                    # Dependency lock file
├── Cargo.toml                    # Project configuration and dependencies
├── readme.md                     # This README file
├── result_data                   # Individual WebSocket results
│   ├── Binance_result_and_data_points.txt
│   ├── Bitfinex_result_and_data_points.txt
│   ├── Bybit_result_and_data_points.txt
│   ├── Gemini_result_and_data_points.txt
│   └── Kraken_result_and_data_points.txt
└── src
    ├── average_btcusd.rs         # Core logic for calculating the BTC/USD average
    ├── counter.rs                # Counter struct to handle price aggregation
    ├── file_operation.rs         # File operations for saving results
    ├── key_counter.rs            # KeyCounter for signature management and verification
    ├── lib.rs                    # Library module
    ├── main.rs                   # Entry point of the application
    └── ws_process.rs             # WebSocket handling and processing logic

3 directories, 17 files
```

### Data Storage
- **Final Aggregated Result**: The final average BTC/USD price is stored in `btcusd_average.txt`.
- **Individual Exchange Data**: Results for each individual exchange (Binance, Kraken, etc.) are stored in the `result_data` directory, with separate files for each exchange.

## Usage

### Cache Mode
In cache mode, the project listens to WebSocket streams for a specified number of seconds and collects BTC/USD prices from different exchanges. It then verifies the data signatures and saves the results to a file.

**Command**:
```bash
cargo run -- --mode=cache --times=9
```
- `--mode=cache`: Specifies that the application should run in cache mode.
- `--times=9`: Specifies the number of seconds to listen to the WebSocket streams.

### Read Mode
In read mode, the application reads previously stored results from the files and prints the data to the terminal.

**Command**:
```bash
cargo run -- --mode=read
```

### Filtering Averages
To search for the average BTC/USD price in the output (since it includes a large number of data points), you can use `grep` to filter for the average.

**Command**:
```bash
cargo run -- --mode=read | grep Average
```

### Running Tests
To run the unit tests for the project, use the following command:
```bash
cargo test
```

### Benchmark Testing
To perform benchmark testing and measure the performance of the application, run:
```bash
cargo bench
```

## Future Improvements
- **More Exchanges**: Additional WebSocket connections from other exchanges can be added to further diversify the data sources.
- **Real-time Dashboard**: An extension to display real-time data in a graphical dashboard could provide better insights.
- **Resilience**: Error handling and retry logic can be implemented to make WebSocket connections more resilient to failures or timeouts.

## Why Rust?
Rust was chosen for this project due to its performance, safety, and concurrency features. The language's strong type system and ownership model help prevent common bugs and ensure thread safety. Rust's async/await syntax and the `tokio` library make it easy to handle multiple WebSocket connections concurrently, providing a high-performance solution for real-time data processing.

# Let's compare it with JavaSript

| Factor                  | **Rust** | **JavaScript** |
|-------------------------|----------|----------------|
| **Performance**          | High     | Moderate       |
| **Concurrency**          | Excellent (multithreading) | Good (event loop, single-threaded) |
| **Memory Safety**        | Strong (ownership system) | Moderate (garbage collection) |
| **System Control**       | High     | Low            |
| **Cryptographic Support**| Strong, safe | Available, but slower |
| **Security**             | Excellent (compile-time checks) | Good, but dynamic typing can introduce bugs |
| **Scalability**          | Excellent (CPU-bound tasks) | Good (I/O-bound tasks) |
