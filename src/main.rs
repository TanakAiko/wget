use clap::Parser;
use wget::{Args, Downloader, WgetResult};

#[tokio::main]
async fn main() -> WgetResult<()> {
    let args = Args::parse();
    let downloader = Downloader::new(args);
    downloader.download_all().await?;
    Ok(())
}