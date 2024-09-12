use tokio_postgres::Client;
use chrono::{DateTime, NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH, Duration};


// Convert NaiveDateTime to SystemTime
fn naive_to_system_time(naive: NaiveDateTime) -> SystemTime {
    let duration_since_epoch = naive.and_utc().timestamp() as u64;
    UNIX_EPOCH + Duration::from_secs(duration_since_epoch)
}

// Convert SystemTime to NaiveDateTime
fn system_time_to_naive(st: SystemTime) -> DateTime<chrono::Utc> {
    let duration_since_epoch = st.duration_since(UNIX_EPOCH).expect("Time went backwards");
    DateTime::from_timestamp(duration_since_epoch.as_secs() as i64, 0).unwrap()
}

pub async fn save_to_db(client: &Client, exchange: &str, price: f64, timestamp: NaiveDateTime) {
    let system_timestamp = naive_to_system_time(timestamp);
    let query = "INSERT INTO prices (exchange, price, timestamp) VALUES ($1, $2, $3)";
    client.execute(query, &[&exchange, &price, &system_timestamp])
        .await
        .expect("Failed to insert data into the database");
    println!("Data successfully saved to the database.");
}

pub async fn read_from_db(client: &Client) {
    let query = "SELECT id, exchange, price, timestamp FROM prices";
    let rows = client.query(query, &[]).await.expect("Failed to read data from the database");

    for row in rows {
        let id: i32 = row.get(0);
        let exchange: String = row.get(1);
        let price: f64 = row.get(2);
        let system_timestamp: SystemTime = row.get(3);
        let timestamp = system_time_to_naive(system_timestamp);

        println!("ID: {}, Exchange: {}, Price: {}, Timestamp: {}", id, exchange, price, timestamp);
    }
}
