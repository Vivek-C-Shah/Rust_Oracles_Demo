use structopt::StructOpt;
use demo_oracles::average_btcusd::cache_mode;
use demo_oracles::file_operation::read_mode;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long)]
    mode: String,

    #[structopt(long)]
    times: Option<u64>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();

    match cli.mode.as_str() {
        //cache mode
        "cache" => match cli.times {
            Some(k) => {
                if k == 0 {
                    println!("time value shoud be greater than 0")
                } else {
                    cache_mode(cli.times.unwrap_or(0)).await
                }
            }
            None => println!("Enter the time"), //but this will be handled by rust itself
        },

        //read mode
        "read" => read_mode().await,
        _ => println!("Invalid mode specified."),
    }
}
