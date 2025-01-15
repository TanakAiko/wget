use clap::Parser;
use wget::{Args, Downloader, WgetResult};

#[tokio::main]
async fn main() -> WgetResult<()> {
    let args = Args::parse();
    let mut downloader = Downloader::new(args).await?;
    downloader.download_all().await?;
    Ok(())
}