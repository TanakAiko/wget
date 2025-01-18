use clap::Parser;
use wget::{Args, Downloader, WgetResult};

#[tokio::main]
async fn main() -> WgetResult<()> {
    let args = Args::parse();
    if let Err(err) = args.validate() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
    let mut downloader = Downloader::new(args).await?;
    downloader.download_all().await?;
    Ok(())
}
