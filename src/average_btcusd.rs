use crate::counter::Counter;
use crate::db_operations::save_to_db;
use crate::key_counter::{verify_signature, KeyCounter};
use crate::ws_process::listen_to_websocket;
// use futures_util::future::try_join_all;
use futures_util::future::try_join_all;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub struct WebSocket {
    ws_url: String,
    msg: Option<String>,
    name: String,
    pointer: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    websocket: std::collections::BTreeMap<String, WebSocket>,
}

pub async fn cache_mode(duration_seconds: u64, client: &tokio_postgres::Client) -> f64 {
    let config_content = std::fs::read_to_string("config.toml").expect("Error reading config file");
    let config: Config = toml::from_str(&config_content).expect("Error parsing config file");

    let res_counter = std::sync::Arc::new(std::sync::Mutex::new(Counter::new())); // for file operations
    let key_counter = std::sync::Arc::new(std::sync::Mutex::new(KeyCounter::new())); // for signatures

    let tasks: Vec<_> = config
        .websocket
        .into_iter()
        .map(|(name, ws_config)| {
            tokio::spawn(listen_to_websocket(
                name.clone(),
                ws_config.ws_url.into(),
                duration_seconds,
                ws_config.msg,
                ws_config.pointer,
                res_counter.clone(),
                key_counter.clone(),
            ))
        })
        .collect();

    let mut sum = 0.0;
    let mut count = 0;

    let results: Vec<_> = try_join_all(tasks)
        .await
        .expect("Error joining tasks")
        .into_iter()
        .flat_map(Result::ok)
        .collect();

    for pair in results {
        let exchange_name = pair.0.clone(); // Get the exchange name
        let price = pair.1;

        let verify = verify_signature(
            key_counter
                .lock()
                .unwrap()
                .public_keys
                .get(&exchange_name)
                .unwrap(),
            price.to_string().as_str(),
            &pair.2,
        );
        
        match verify {
            true => {
                if price != 0.0 {
                    sum += price;
                    count += 1;

                    // Save each exchange's price to the database
                    let timestamp = chrono::Utc::now().naive_utc();
                    save_to_db(client, &exchange_name, price, timestamp).await;
                    println!(
                        "Saved data for {}: Price: {}, Timestamp: {}",
                        exchange_name, price, timestamp
                    );
                }
            }
            false => eprintln!("Signature verification failed for exchange: {}", exchange_name),
        }
    }

    // Calculate and return the average price
    let average_price = if count > 0 { sum / count as f64 } else { 0.0 };

    // Save the average price to the database
    let timestamp = chrono::Utc::now().naive_utc();
    save_to_db(client, "average", average_price, timestamp).await;
    println!("Successfully saved the average price: {} to the database.", average_price);

    average_price
}
