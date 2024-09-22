// src/main.rs

use std::process::Command;
use std::fs;
use std::env;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let set_default_shell = args.get(1).map_or(false, |arg| arg == "--set-default");

    // Create a client with rustls TLS backend
    let client = Client::builder()
        .user_agent("setup_nu")
        .build()?;

    // Fetch the latest Nushell release information from GitHub API
    let response = client
        .get("https://api.github.com/repos/nushell/nushell/releases/latest")
        .send()?
        .json::<Release>()?;

    println!("Latest Nushell version: {}", response.tag_name);

    // Find the asset that matches the system's architecture
    let target_asset = response.assets.iter().find(|asset| {
        asset.name.contains("linux")
            && asset.name.ends_with(".tar.gz")
            && !asset.name.contains("musl")
    }).expect("Could not find a suitable Nushell asset for Linux.");

    println!("Downloading {}", target_asset.name);

    // Download the tarball
    let mut response = client.get(&target_asset.browser_download_url).send()?;
    let mut file = fs::File::create("nushell.tar.gz")?;
    response.copy_to(&mut file)?;

    // Extract the tarball
    let output = Command::new("tar")
        .args(&["-xzf", "nushell.tar.gz"])
        .output()
        .expect("Failed to extract Nushell");

    if !output.status.success() {
        panic!("Failed to extract Nushell: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Move the binary to ~/.local/bin
    fs::create_dir_all(format!("{}/.local/bin", env::var("HOME")?))?;
    fs::copy("./nu/nu", format!("{}/.local/bin/nu", env::var("HOME")?))?;

    println!("Nushell installed successfully!");

    // Set Nushell as the default shell if requested
    if set_default_shell {
        println!("Setting Nushell as the default shell...");

        let output = Command::new("chsh")
            .args(&["-s", &format!("{}/.local/bin/nu", env::var("HOME")?), &env::var("USER")?])
            .output()
            .expect("Failed to set Nushell as the default shell");

        if !output.status.success() {
            panic!("Failed to set Nushell as the default shell: {}", String::from_utf8_lossy(&output.stderr));
        }

        println!("Nushell is now set as the default shell!");
    }

    Ok(())
}
