use std::process::Command;
use std::fs;
use std::env;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::Write;
use std::path::Path;
use std::process::exit;

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
    let args: Vec<String> = env::args().collect();
    let set_default_shell = args.get(1).map_or(false, |arg| arg == "--set-default");

    // Determine system architecture and OS
    let architecture = match env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "arm" => "arm",
        arch => {
            eprintln!("Unsupported architecture: {}", arch);
            exit(1);
        }
    };

    let os = match env::consts::OS {
        "linux" => "unknown-linux-gnu",
        "macos" => "apple-darwin",
        "windows" => "pc-windows-msvc",
        os => {
            eprintln!("Unsupported operating system: {}", os);
            exit(1);
        }
    };

    // Create the expected filename part
    let expected_filename_part = format!("{}-{}", architecture, os);

    let client = Client::builder()
        .user_agent("setup_nu")
        .build()?;

    // Fetch the latest release info
    let response = client
        .get("https://api.github.com/repos/nushell/nushell/releases/latest")
        .send()?
        .json::<Release>()?;

    println!("Latest Nushell version: {}", response.tag_name);

    // Find the correct asset based on architecture and OS
    let target_asset = response.assets.iter().find(|asset| {
        asset.name.contains(&expected_filename_part) && asset.name.ends_with(".tar.gz")
    }).expect("Could not find a suitable Nushell asset for this architecture and OS.");

    println!("Downloading {}", target_asset.name);

    // Download the tarball
    let mut response = client.get(&target_asset.browser_download_url).send()?;
    let tarball_path = target_asset.name.clone();
    let mut file = fs::File::create(&tarball_path)?;
    response.copy_to(&mut file)?;

    // Extract the tarball
    let output = Command::new("tar")
        .args(&["-xzf", &tarball_path])
        .output()
        .expect("Failed to extract Nushell");

    if !output.status.success() {
        eprintln!(
            "Failed to extract Nushell: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    // Find the extracted binary
    let extracted_dir = tarball_path.strip_suffix(".tar.gz").unwrap();
    let nu_binary_path = Path::new(extracted_dir).join("nu");

    if !nu_binary_path.exists() {
        eprintln!("Failed to find the extracted Nushell binary at {:?}", nu_binary_path);
        exit(1);
    }

    // Create the ~/.local/bin directory if it doesn't exist
    let local_bin_dir = format!("{}/.local/bin", env::var("HOME")?);
    fs::create_dir_all(&local_bin_dir)?;

    // Move the binary to ~/.local/bin
    let destination = format!("{}/nu", local_bin_dir);
    fs::copy(nu_binary_path, &destination)?;

    println!("Nushell installed successfully at {}", &destination);

    // Set Nushell as the default shell if requested
    if set_default_shell {
        println!("Setting Nushell as the default shell...");

        let output = Command::new("chsh")
            .args(&["-s", &destination, &env::var("USER")?])
            .output()
            .expect("Failed to set Nushell as the default shell");

        if !output.status.success() {
            eprintln!(
                "Failed to set Nushell as the default shell: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }

        println!("Nushell is now set as the default shell!");
    }

    Ok(())
}

