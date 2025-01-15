use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URLs to download (space separated)
    #[arg(index = 1, num_args = 1.., required = true)]
    pub urls: Vec<String>,

    /// Save files under different names
    #[arg(short = 'O')]
    pub output: Option<String>,

    /// Save files in a specific directory
    #[arg(short = 'P')]
    pub path: Option<String>,

    /// Download in background
    #[arg(short = 'B')]
    pub background: bool,

    /// Rate limit (e.g., "200k" or "2M")
    #[arg(long = "rate-limit")]
    pub rate_limit: Option<String>,
}