use crate::{mirror::WebsiteMirror, utils, Args, WgetResult};
use chrono::Local;
use futures_util::StreamExt;
use indicatif::MultiProgress;
use reqwest;
use std::env;
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::task;

pub struct Downloader {
    args: Args,
    client: reqwest::Client,
    output_file: Option<File>,
}

impl Clone for Downloader {
    fn clone(&self) -> Self {
        Downloader {
            args: self.args.clone(),
            client: self.client.clone(),
            output_file: None, // ignore output_file in the copy
        }
    }
}

impl Downloader {
    pub async fn new(args: Args) -> WgetResult<Self> {
        let output_file = if args.background {
            Some(File::create("wget-log").await?)
        } else {
            None
        };

        Ok(Self {
            args,
            client: reqwest::Client::new(),
            output_file,
        })
    }

    async fn logln(&mut self, message: &str) -> WgetResult<()> {
        if let Some(file) = &mut self.output_file {
            file.write_all(format!("{}\n", message).as_bytes()).await?;
            file.flush().await?;
        } else {
            println!("{}", message);
        }
        Ok(())
    }

    async fn log(&mut self, message: &str) -> WgetResult<()> {
        if let Some(file) = &mut self.output_file {
            file.write_all(format!("{}", message).as_bytes()).await?;
            file.flush().await?;
        } else {
            print!("{}", message);
        }
        Ok(())
    }

    pub async fn download_all(&mut self) -> WgetResult<()> {
        if self.args.background {
            println!("Output will be written to \"wget-log\"");
        }

        if self.args.mirror {
            self.mirror_websites().await?;
        } else {
            let start_time = Local::now();
            self.logln(&format!(
                "start at {}",
                start_time.format("%Y-%m-%d %H:%M:%S")
            ))
            .await?;

            let m = if !self.args.background {
                Some(MultiProgress::new())
            } else {
                None
            };

            let rate_limit = self.parse_rate_limit()?;
            //println!("rate_limit: {:?}", rate_limit);

            if let Some(input_file) = &self.args.input_file {
                let urls = Self::read_urls_from_file(input_file).await?;
                let mut tasks = Vec::new();

                for url in urls {
                    let mut downloader = self.clone(); // clone `Self` for each task
                    let rate_limit = rate_limit.clone();

                    let task = task::spawn(async move {
                        if let Err(e) = downloader.download_file(&url, rate_limit, None).await {
                            eprintln!("Failed to download {}: {}", url, e);
                        }
                    });
                    tasks.push(task);
                }

                for task in tasks {
                    task.await.unwrap(); // wait until all the tasks are completed
                }
            }

            // créer un clone des URLs pour éviter le problème de borrowing
            let urls: Vec<String> = self.args.urls.clone();

            for (_, url) in urls.iter().enumerate() {
                self.download_file(url, rate_limit, m.as_ref()).await?;
            }

            self.logln(&format!(
                "finished at {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            ))
            .await?;
        }
        Ok(())
    }

    async fn mirror_websites(&self) -> WgetResult<()> {
        for url in &self.args.urls {
            println!("Mirroring website: {}", url);
            let mut mirror = WebsiteMirror::new(
                url.clone(),
                self.args.get_rejected_extensions(),
                self.args.get_excluded_paths(),
                self.args.convert_links,
            )?;
            mirror.start().await?;
        }
        Ok(())
    }

    async fn download_file(
        &mut self,
        url: &str,
        rate_limit: Option<u64>,
        progress_bars: Option<&MultiProgress>,
    ) -> WgetResult<()> {
        if self.args.input_file.is_none() {
            self.log(&format!("sending request, awaiting response... "))
                .await?;
        }

        let response = self.client.get(url).send().await?;

        let status = response.status();
        if self.args.input_file.is_none() {
            self.logln(&format!("status {}", status)).await?;
        }

        if !status.is_success() {
            return Err(format!("Failed with status: {}", status).into());
        }

        let total_size = response.content_length().unwrap_or(0);
        self.logln(&format!(
            "content size: {} [~{}]",
            total_size,
            utils::format_size(total_size)
        ))
        .await?;

        let mut filename = match &self.args.output {
            Some(name) => format!("{}", name),
            None => utils::extract_filename_from_url(url),
        };

        let mut dest_path = match &self.args.path {
            Some(p) => PathBuf::from(p).join(&filename),
            None => PathBuf::from(&filename),
        };

        // check if a file with this name already exists
        let mut unique_index = 1;
        while dest_path.exists() {
            filename = match &self.args.output {
                Some(name) => {
                    utils::add_suffix_before_extension(&name, &format!("_{}", unique_index))
                }
                None => utils::add_suffix_before_extension(
                    &utils::extract_filename_from_url(url),
                    &format!("_{}", unique_index),
                ),
            };

            dest_path = match &self.args.path {
                Some(p) => PathBuf::from(p).join(&filename),
                None => PathBuf::from(&filename),
            };

            unique_index += 1;
        }

        if self.args.input_file.is_none() {
            self.logln(&format!("saving file to: {}", dest_path.display()))
                .await?;
        }

        let pb = if let Some(mp) = progress_bars {
            let pb = mp.add(utils::create_progress_bar(total_size));
            pb.set_prefix(format!("[{}]", filename));
            Some(pb)
        } else {
            None
        };

        let mut downloaded: u64 = 0;

        if dest_path.starts_with("~") {
            if let Ok(home_dir) = env::var("HOME") {
                let dest_path_str = dest_path.to_string_lossy();
                dest_path = PathBuf::from(format!(
                    "{}/{}",
                    home_dir,
                    dest_path_str.trim_start_matches('~')
                ));
            }
        }

        let mut file = File::create(&dest_path).await?;
        let mut stream = response.bytes_stream();
        let mut last_check = Instant::now();
        let mut bytes_since_last_check: u64 = 0;
        let mut speed = 0.0;
        let delta_time = 0.3;

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let chunk_size = chunk.len() as u64;

            bytes_since_last_check += chunk_size;
            downloaded += chunk_size;

            let elapsed = last_check.elapsed().as_secs_f64();

            if let Some(max_speed) = rate_limit {
                // println!(
                //     "bytes_since_last_check: {}; max_speed: {}",
                //     bytes_since_last_check, max_speed
                // );
                if bytes_since_last_check >= max_speed - 2 * chunk_size {
                    if elapsed >= delta_time {
                        speed = bytes_since_last_check as f64;
                        bytes_since_last_check = 0;
                        last_check = Instant::now();
                    } else {
                        let sleep_time = delta_time - elapsed;
                        tokio::time::sleep(tokio::time::Duration::from_secs_f64(sleep_time)).await;
                    }
                }
            } else {
                if elapsed >= delta_time {
                    speed = bytes_since_last_check as f64;
                    bytes_since_last_check = 0;
                    last_check = Instant::now();
                }
            }

            file.write_all(&chunk).await?;

            if let Some(pb) = &pb {
                let percentage = (downloaded as f64 / total_size as f64) * 100.0;
                pb.set_message(format!(
                    "{:.2}%, Speed: {}/s",
                    percentage,
                    utils::format_size(speed as u64)
                ));
                pb.set_position(downloaded);
            }
        }

        if let Some(pb) = &pb {
            pb.finish_with_message("completed");
        }

        self.logln(&format!("\nDownloaded [{}]", url)).await?;
        Ok(())
    }

    fn parse_rate_limit(&self) -> WgetResult<Option<u64>> {
        if let Some(limit) = &self.args.rate_limit {
            let mut chars = limit.chars();
            let mut num_str = String::new();
            let mut unit = String::new();

            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    num_str.push(c);
                } else {
                    unit.push(c);
                    unit.extend(chars);
                    break;
                }
            }

            let number: u64 = num_str.parse()?;

            let bytes_per_sec = match unit.to_lowercase().as_str() {
                "k" => number * 1000,
                "m" => number * 1000 * 1000,
                _ => return Err("Invalid rate limit unit (use k or M)".into()),
            };

            Ok(Some(bytes_per_sec))
        } else {
            Ok(None)
        }
    }

    async fn read_urls_from_file(file_path: &str) -> WgetResult<Vec<String>> {
        // Open the specified file
        let file = File::open(file_path).await?;
        let reader = BufReader::new(file);

        // Initializes a vector to store URLs
        let mut urls = Vec::new();

        // read the line file by line
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            let trimmed = line.trim();
            // ignore the empty lines
            if !trimmed.is_empty() {
                urls.push(trimmed.to_string());
            }
        }

        Ok(urls)
    }
}
