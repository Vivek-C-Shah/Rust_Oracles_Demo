# Demo Oracle Home Test

## Project Overview
This project listens to live Bitcoin-to-USD (BTC/USD) price streams from multiple WebSocket connections across different cryptocurrency exchanges. It collects price data, verifies cryptographic signatures to ensure data integrity, and calculates the average price of Bitcoin in USD.

The project stores the aggregated results and individual exchange results in a PostgreSQL database for future analysis. It also has the option to store results in files for backup purposes.

## Features
- **Multiple WebSocket Connections**: Connects to multiple cryptocurrency exchanges (like Binance, Kraken, etc.) to fetch live BTC/USD prices.
- **Cryptographic Signature Verification**: Uses cryptographic verification to ensure the integrity of the data.
- **Concurrent Data Fetching**: Efficiently handles multiple WebSocket streams using Rust's async runtime.
- **Price Aggregation**: Calculates the average BTC/USD price from all streams and stores the result.
- **Database Integration**: Stores both the individual exchange data and aggregated results in a PostgreSQL database.
- **File Backup**: Provides the option to store results in files for further analysis or backup.
- **Benchmarking**: Performance testing using Criterion.

## Dependencies
The project relies on the following key dependencies:
- **[tokio](https://crates.io/crates/tokio)** and **[tokio-tungstenite](https://crates.io/crates/tokio-tungstenite)**: Asynchronous runtime and WebSocket library for handling concurrent connections.
- **[tokio-postgres](https://crates.io/crates/tokio-postgres)**: PostgreSQL client for async database queries.
- **[k256](https://crates.io/crates/k256)**: Library for elliptic curve signature verification (used for verifying the data signatures).
- **[criterion](https://crates.io/crates/criterion)**: Benchmarking tool for performance testing.
- **[dotenv](https://crates.io/crates/dotenv)**: For loading environment variables, including the database connection string.

## Project Structure
```
.
├── benches
│   └── bench_test.rs             # Benchmarking tests
├── Cargo.lock                    # Dependency lock file
├── Cargo.toml                    # Project configuration and dependencies
├── readme.md                     # This README file
├── src
│   ├── average_btcusd.rs         # Core logic for calculating the BTC/USD average
│   ├── counter.rs                # Counter struct to handle price aggregation
│   ├── db_operations.rs          # Database operations for saving results
│   ├── file_operation.rs         # File operations for saving results (optional backup)
│   ├── key_counter.rs            # KeyCounter for signature management and verification
│   ├── lib.rs                    # Library module
│   ├── main.rs                   # Entry point of the application
│   └── ws_process.rs             # WebSocket handling and processing logic
```

### Data Storage
- **Final Aggregated Result**: The final average BTC/USD price is stored in a PostgreSQL database under a record labeled `average`.
- **Individual Exchange Data**: Results for each individual exchange (Binance, Kraken, etc.) are stored in the PostgreSQL database, with the `exchange` field identifying the source of the data.
- **File Backup (Optional)**: Results can also be stored in backup files for additional analysis, though the database is the primary storage method.

## Usage

### Database Configuration
Before running the application, ensure that your PostgreSQL database is set up. You can use the following SQL commands to create the necessary database and table:

```sql
CREATE DATABASE rust_oracle;
\c rust_oracle
CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    exchange VARCHAR(50),
    price NUMERIC,
    timestamp TIMESTAMP
);
```

Ensure your `.env` file contains the `DATABASE_URL` with the correct connection string:

```
DATABASE_URL="postgres://username:password@localhost:5432/rust_oracle"
```

### Cache Mode
In cache mode, the project listens to WebSocket streams for a specified number of seconds and collects BTC/USD prices from different exchanges. It verifies the data signatures and stores the results both per exchange and as an aggregated average in the PostgreSQL database.

**Command**:
```bash
cargo run -- --mode=cache --times=9
```
- `--mode=cache`: Specifies that the application should run in cache mode.
- `--times=9`: Specifies the number of seconds to listen to the WebSocket streams.

### Read Mode
In read mode, the application reads previously stored results from the PostgreSQL database and prints the data to the terminal.

**Command**:
```bash
cargo run -- --mode=read
```

### Filtering Averages
To search for the average BTC/USD price in the output, you can use `grep` to filter for the average.

**Command**:
```bash
cargo run -- --mode=read | grep average
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
- **Advanced Querying**: Add more advanced querying options for retrieving historical data from the PostgreSQL database.

## Why Rust?
Rust was chosen for this project due to its performance, safety, and concurrency features. The language's strong type system and ownership model help prevent common bugs and ensure thread safety. Rust's async/await syntax and the `tokio` library make it easy to handle multiple WebSocket connections concurrently, providing a high-performance solution for real-time data processing.

# Comparison with JavaScript

| Factor                  | **Rust**                         | **JavaScript**               |
|-------------------------|----------------------------------|------------------------------|
| **Performance**          | High (low-level control)         | Moderate                     |
| **Concurrency**          | Excellent (multithreading)       | Good (event loop, single-threaded) |
| **Memory Safety**        | Strong (ownership system)        | Moderate (garbage collection) |
| **System Control**       | High                             | Low                          |
| **Cryptographic Support**| Strong, safe                     | Available, but slower         |
| **Security**             | Excellent (compile-time checks)  | Good, but dynamic typing can introduce bugs |
| **Scalability**          | Excellent (CPU-bound tasks)      | Good (I/O-bound tasks)        |
