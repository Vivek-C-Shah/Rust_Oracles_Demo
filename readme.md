# SupraOracles Home Test
### It will listen to multiple(5) sockets for given number of seconds and it will get average individually and then it will verifies the signatures and calculates the average 

## Most used Dependencies
- tokio and tokio-tungstenite 
- k256 for signatures
- criterion
## Project structure
```
.
├── benches
│   └── bench_test.rs
├── btcusd_average.txt
├── Cargo.lock
├── Cargo.toml
├── readme.md
├── result_data
│   ├── Binance_result_and_data_points.txt
│   ├── Bitfinex_result_and_data_points.txt
│   ├── Bybit_result_and_data_points.txt
│   ├── Gemini_result_and_data_points.txt
│   └── Kraken_result_and_data_points.txt
└── src
    ├── average_btcusd.rs
    ├── counter.rs
    ├── file_operation.rs
    ├── key_counter.rs
    ├── lib.rs
    ├── main.rs
    └── ws_process.rs

3 directories, 17 files
```
### Here  Final result stored in "btcusd_avg.txt" and individual averages are stored in result_data directory

## Commands
Cache mode: It will listen to websockets for given number of times(seconds) for the USD
prices of BTC and save the result to a file
```
cargo run  -- --mode=cache --times=9
```
Read mode: It will just read from the file and print the values to the terminal

``` 
cargo run  -- --mode=read
```
to search Average in result as it includes Average and huge number of data points
```
cargo run -- --mode=read | grep Average
```
Test: 
```
cargo test
```
Benchmark Testing:
```
cargo bench
```


