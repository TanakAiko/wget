use std::error::Error;

pub mod args;
pub mod downloader;
pub mod mirror;

pub use args::Args;
pub use downloader::Downloader;

// Type alias pour simplifier la gestion des erreurs
pub type WgetResult<T> = Result<T, Box<dyn Error>>;

// Fonctions utilitaires qui peuvent être utilisées dans tout le projet
pub mod utils {
    use indicatif::{ProgressBar, ProgressStyle};

    pub fn extract_filename_from_url(url: &str) -> String {
        url.split('/').last().unwrap_or("download").to_string()
    }

    pub fn format_size(size: u64) -> String {
        if size >= 1_000_000 {
            format!("{:.2}MB", size as f64 / 1_000_000.0)
        } else if size >= 1_000 {
            format!("{:.2}KB", size as f64 / 1_000.0)
        } else {
            format!("{}B", size)
        }
    }

    pub fn create_progress_bar(total_size: u64) -> ProgressBar {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix:.bold} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"));
        pb
    }

    pub fn add_suffix_before_extension(filename: &str, suffix: &str) -> String {
        if let Some(pos) = filename.rfind('.') {
            let (name, ext) = filename.split_at(pos);
            format!("{}{}{}", name, suffix, ext)
        } else {
            format!("{}{}", filename, suffix)
        }
    }
}
