use tokio::fs::File;
use tokio::io::AsyncWriteExt;
pub async fn save_to_file(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}
pub async fn read_mode() {
    let file_path = "btcusd_average.txt";

    if let Ok(content) = tokio::fs::read_to_string(file_path).await {
        println!("{}", content);
    } else {
        println!("Error reading from file.");
    }
}
