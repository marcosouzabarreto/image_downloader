# Image Downloader CLI

A simple Rust CLI tool that downloads images from given URLs and saves them in a folder named with the current timestamp.

Features

- Multiple URL Support: Pass one or more image URLs as command-line arguments.
- Timestamped Directories: Downloads are organized into a directory named with the current timestamp.
- Simple Usage: Easy to run with minimal setup.

Installation

    Clone the repository:

git clone https://github.com/marcosouzabarreto/image_downloader.git
cd image_downloader

Build the project:

    cargo build --release

Usage

Run the CLI by passing one or more image URLs as arguments. For example:

cargo run -- "https://example.com/image1.jpg" "https://example.com/image2.png"

Note for fish shell users: Wrap your URLs in quotes to avoid wildcard expansion issues.
