use demo_oracles::average_btcusd::cache_mode;
use demo_oracles::db_operations::read_from_db;
use dotenv::dotenv;
use std::env;
use structopt::StructOpt;
use tokio_postgres::NoTls;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long)]
    mode: String,

    #[structopt(long)]
    times: Option<u64>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli: Cli = Cli::from_args();

    // Use .env file to fetch the database URL or fall back to a default string
    let db_url = env::var("DATABASE_URL").unwrap();
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
        .await
        .expect("Failed to connect to database");

    // Spawn a separate task for the connection to handle async queries
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    match cli.mode.as_str() {
        // Cache mode: connect to WebSocket(s), calculate the average price, and store the result in the database
        "cache" => match cli.times {
            Some(duration_seconds) => {
                if duration_seconds == 0 {
                    println!("time value should be greater than 0")
                } else {
                    // Pass the `client` reference to cache_mode so it can save data
                    cache_mode(duration_seconds, &client).await;
                }
            }
            None => println!("Please specify the time duration"),
        },

        // Read mode: read from the database
        "read" => read_from_db(&client).await,

        _ => println!("Invalid mode specified."),
    }
}
