use crate::{Args, WgetResult, utils};
use chrono::Local;
use reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use futures_util::StreamExt;
use indicatif::MultiProgress;

pub struct Downloader {
    args: Args,
    client: reqwest::Client,
}

impl Downloader {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_all(&self) -> WgetResult<()> {
        println!("start at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        
        let m = MultiProgress::new();
        let mut handles = Vec::new();

        // Créer une tâche de téléchargement pour chaque URL
        for (index, url) in self.args.urls.iter().enumerate() {
            let client = self.client.clone();
            let url = url.clone();
            let path = self.args.path.clone();
            let output = self.args.output.clone();
            let mp = m.clone();

            let handle = tokio::spawn(async move {
                let result = Self::download_file(client, url.clone(), path, output, mp, index).await;
                if let Err(e) = result {
                    eprintln!("Error downloading {}: {}", url, e);
                }
            });

            handles.push(handle);
        }

        // Attendre que tous les téléchargements soient terminés
        for handle in handles {
            handle.await?;
        }

        println!("finished at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        Ok(())
    }

    async fn download_file(
        client: reqwest::Client,
        url: String,
        path: Option<String>,
        output: Option<String>,
        mp: MultiProgress,
        index: usize,
    ) -> WgetResult<()> {
        println!("sending request for {}, awaiting response...", url);
        let response = client.get(&url).send().await?;
        
        let status = response.status();
        print!("status {} ", status);
        if !status.is_success() {
            return Err(format!("Failed with status: {}", status).into());
        }
        println!("OK");

        let total_size = response.content_length().unwrap_or(0);
        println!("content size: {} [~{}]", total_size, utils::format_size(total_size));

        // Construire le chemin de destination
        let filename = match &output {
            Some(name) => format!("{}_{}", name, index),
            None => utils::extract_filename_from_url(&url),
        };

        let dest_path = match &path {
            Some(p) => PathBuf::from(p).join(&filename),
            None => PathBuf::from(&filename),
        };

        println!("saving file to: {}", dest_path.display());

        // Créer la barre de progression
        let pb = mp.add(utils::create_progress_bar(total_size));
        pb.set_prefix(format!("[{}]", filename));

        // Télécharger le fichier par chunks
        let mut downloaded: u64 = 0;
        let mut file = File::create(&dest_path).await?;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("completed");
        println!("\nDownloaded [{}]", url);
        
        Ok(())
    }
}