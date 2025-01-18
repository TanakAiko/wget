use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URLs to download (space separated)
    #[arg(index = 1, num_args = 1..)]
    pub urls: Vec<String>,

    /// Input file containing URLs to download
    #[arg(short = 'i')]
    pub input_file: Option<String>,

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

    /// Mirror website
    #[arg(long = "mirror")]
    pub mirror: bool,

    /// File types to reject (comma-separated)
    #[arg(short = 'R', long = "reject")]
    pub reject: Option<String>,

    /// Directories to exclude (comma-separated)
    #[arg(short = 'X', long = "exclude")]
    pub exclude: Option<String>,

    /// Convert links for offline viewing
    #[arg(long = "convert-links")]
    pub convert_links: bool,
}

impl Args {
    pub fn get_rejected_extensions(&self) -> HashSet<String> {
        self.reject
            .as_ref()
            .map(|r| r.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_excluded_paths(&self) -> HashSet<String> {
        self.exclude
            .as_ref()
            .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.urls.is_empty() && self.input_file.is_none() {
            return Err("wget: missing URL\nUsage: wget [OPTION]... [URL]...\n\nTry `wget --help` for more options.".into());
        }
        Ok(())
    }
}
