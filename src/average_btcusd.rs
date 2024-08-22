use crate::counter::Counter;
use crate::file_operation::save_to_file;
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

pub async fn cache_mode(duration_seconds: u64) {
    let config_content = std::fs::read_to_string("config.toml").expect("Error reading config file");
    let config: Config = toml::from_str(&config_content).expect("Error parsing config file");
    //----------------------------------------------------------------------------------------------------------------------

    let res_counter = std::sync::Arc::new(std::sync::Mutex::new(Counter::new())); //for file operations
    let key_counter = std::sync::Arc::new(std::sync::Mutex::new(KeyCounter::new())); //for signatures

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
        let verify = verify_signature(
            key_counter
                .lock()
                .unwrap()
                .public_keys
                .get(&pair.0)
                .unwrap(),
            pair.1.to_string().as_str(),
            &pair.2,
        ); //verifying the signature-------------------
        match verify {
            true => {
                if pair.1 != 0.0 {
                    sum += pair.1;
                    count += 1;
                }
            }
            false => eprintln!("Signature verification failed"),
        }
    }
    let average = sum / count as f64;
    //format to store the data in btcusd_average.txt--------------------------this is the aggregated result
    let content = format!(
        "The Average USD price of BTC is: {} \n {:?}",
        average,
        res_counter.lock().unwrap().data
    );
    //---------------------------------------------------------------------------------------------------
    let file_path = format!("btcusd_average.txt");
    let _ = save_to_file(file_path.as_str(), &content).await;
    println!(
        "Total Cache complete. The average USD price of BTC is: {}",
        average
    );
}
