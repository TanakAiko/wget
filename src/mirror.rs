use crate::WgetResult;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use url::Url;

pub struct WebsiteMirror {
    client: Client,
    base_url: Url,
    output_dir: PathBuf,
    visited_urls: HashSet<String>,
    rejected_extensions: HashSet<String>,
    excluded_paths: HashSet<String>,
    convert_links: bool,
    queue: VecDeque<String>,
}

impl WebsiteMirror {
    pub fn new(
        url: String,
        rejected_extensions: HashSet<String>,
        excluded_paths: HashSet<String>,
        convert_links: bool,
    ) -> WgetResult<Self> {
        let base_url = Url::parse(&url)?;
        let domain = base_url.host_str().ok_or("Invalid URL: no host")?.to_string();
        let output_dir = PathBuf::from(&domain);

        Ok(Self {
            client: Client::new(),
            base_url,
            output_dir,
            visited_urls: HashSet::new(),
            rejected_extensions,
            excluded_paths,
            convert_links,
            queue: VecDeque::from([url]),
        })
    }

    pub async fn start(&mut self) -> WgetResult<()> {
        // Créer le répertoire de sortie
        fs::create_dir_all(&self.output_dir).await?;

        while let Some(url) = self.queue.pop_front() {
            if self.visited_urls.contains(&url) {
                continue;
            }

            if let Err(e) = self.process_url(&url).await {
                eprintln!("Error processing {}: {}", url, e);
            }

            self.visited_urls.insert(url);
        }

        Ok(())
    }

    async fn process_url(&mut self, url: &str) -> WgetResult<()> {
        println!("Processing: {}", url);

        // Vérifier si l'URL doit être exclue
        if self.should_exclude(url) {
            return Ok(());
        }

        let response = self.client.get(url).send().await?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Obtenir le chemin relatif pour la sauvegarde
        let relative_path = self.get_relative_path(url)?;
        let full_path = self.output_dir.join(&relative_path);

        // Créer les répertoires parents si nécessaire
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        
        if content_type.contains("text/html") {
            // Traiter le HTML
            let html_content = response.text().await?;
            let processed_html = self.process_html(&html_content, url)?;
            let mut file = File::create(&full_path).await?;
            file.write_all(processed_html.as_bytes()).await?;
        } else  if content_type.contains("text/css") || content_type.contains("application/javascript") {
            let content = response.bytes().await?;
            let mut file = File::create(&full_path).await?;
            file.write_all(&content).await?;
        } else {
            // Télécharger le fichier directement
            let content = response.bytes().await?;
            let mut file = File::create(&full_path).await?;
            file.write_all(&content).await?;
        }
        
       
        Ok(())
    }

    

    fn process_html(&mut self, html: &str, base_url: &str) -> WgetResult<String> {
        let document = Html::parse_document(html);
        let base_url = Url::parse(base_url)?;
    
        let selectors = [
            (Selector::parse("a[href]").unwrap(), "href"),
            (Selector::parse("link[href]").unwrap(), "href"),
            (Selector::parse("img[src]").unwrap(), "src"),
            (Selector::parse("script[src]").unwrap(), "src"), // Gérer les fichiers JS
            (Selector::parse("link[rel='stylesheet']").unwrap(), "href"),
            (Selector::parse("style").unwrap(), "textContent"),
        ];
    
        let mut processed_html = html.to_string();
    
        for (selector, attr) in selectors.iter() {
            for element in document.select(selector) {
                if let Some(link) = element.value().attr(attr) {
                    if let Ok(absolute_url) = base_url.join(link) {
                        let url_str = absolute_url.as_str();
    
                        // Ajouter l'URL à la queue si elle appartient au même domaine
                        if absolute_url.host() == base_url.host() {
                            self.queue.push_back(url_str.to_string());
                        }
    
                        // Convertir les liens si demandé
                        if self.convert_links {
                            let relative_path = self.get_relative_path(url_str)
                                .unwrap_or_else(|_| link.to_string().into());
                            processed_html = processed_html.replace(link, relative_path.to_str().unwrap_or(link));
                        }
                    }
                }
            }
        }
    
        Ok(processed_html)
    }
    

    fn should_exclude(&self, url: &str) -> bool {
        // Vérifier les extensions rejetées
        if let Ok(url) = Url::parse(url) {
            if let Some(path) = url.path_segments() {
                if let Some(last) = path.last() {
                    if let Some(ext) = Path::new(last).extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if self.rejected_extensions.contains(ext_str) {
                                return true;
                            }
                        }
                    }
                }
            }

            // Vérifier les chemins exclus
            for excluded in &self.excluded_paths {
                if url.path().starts_with(excluded) {
                    return true;
                }
            }
        }

        false
    }

    fn get_relative_path(&self, url: &str) -> WgetResult<PathBuf> {
        let url = Url::parse(url)?;
        let path = url.path();
        let path = if path == "/" {
            "index.html".to_string()  // Convertir en String
        } else if path.ends_with('/') {
            format!("{}index.html", path)
        } else if !path.contains('.') {
            format!("{}/index.html", path)
        } else {
            path.to_string()
        };
        Ok(PathBuf::from(&path[1..]))
    }
}