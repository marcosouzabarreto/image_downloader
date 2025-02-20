use clap::Parser;
use chrono::Local;
use reqwest;
use std::{fs, io::Write, error::Error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.urls.is_empty() {
        eprintln!("Usage: image_downloader <url1> <url2> ...");
        return Ok(());
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let dir_name = format!("images_from_{}", timestamp);
    fs::create_dir_all(&dir_name)?;

    for url in args.urls {
        println!("Downloading: {}", url);

        let response = reqwest::get(&url).await?;
        if !response.status().is_success() {
            eprintln!("Failed to download {}: HTTP {}", url, response.status());
            continue;
        }

        let bytes = response.bytes().await?;

        let file_name = url
            .split('/')
            .last()
            .filter(|name| !name.is_empty())
            .unwrap_or("downloaded_image");
        let file_path = format!("{}/{}", dir_name, file_name);

        let mut file = fs::File::create(&file_path)?;
        file.write_all(&bytes)?;
        println!("Saved to: {}", file_path);
    }

    Ok(())
}

