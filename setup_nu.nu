# setup_nu.nu

# Check for dependencies
if not (which wget) {
    echo "wget is required but not installed. Please install it first."
    exit 1
}

if not (which tar) {
    echo "tar is required but not installed. Please install it first."
    exit 1
}

if not (which gpg) {
    echo "gpg is required but not installed. Please install it first."
    exit 1
}

# Determine system architecture
let arch = ($nu.os.arch | str trim)
let os_name = ($nu.os.name | str trim)

# Map the architecture and OS to the appropriate Nushell download target
let platform = (
    if $arch == "x86_64" && $os_name == "linux" { "x86_64-unknown-linux-gnu" }
    else if $arch == "aarch64" && $os_name == "linux" { "aarch64-unknown-linux-gnu" }
    else if $arch == "x86_64" && $os_name == "darwin" { "x86_64-apple-darwin" }
    else if $arch == "aarch64" && $os_name == "darwin" { "aarch64-apple-darwin" }
    else if $arch == "x86_64" && $os_name == "windows" { "x86_64-pc-windows-msvc" }
    else { 
        echo "Unsupported architecture/OS combination: $arch / $os_name"
        exit 1
    }
)

# Fetch the latest release information from GitHub
let release_url = "https://api.github.com/repos/nushell/nushell/releases/latest"
let release_info = (fetch $release_url | from json)

let version = $release_info.tag_name
echo $"Latest Nushell version: ($version)"

# Find the asset URL matching the platform
let asset_url = ($release_info.assets | where name =~ $platform | get browser_download_url | first)
if ($asset_url | empty?) {
    echo "Could not find a suitable download for this platform ($platform)."
    exit 1
}

echo $"Downloading Nushell from ($asset_url)"

# Download the tarball
let tarball = $"nu-($version)-($platform).tar.gz"
wget $asset_url -O $tarball

# Extract the tarball
echo "Extracting Nushell..."
tar -xzf $tarball

# Move the binary to ~/.local/bin
let home_dir = ($nu.env.HOME | str trim)
let dest_dir = $"($home_dir)/.local/bin"
mkdir -p $dest_dir
mv nu/nu $"($dest_dir)/nu"

# Add ~/.local/bin to the PATH if it's not already there
if ($nu.env.PATH | split row ":" | where $it == $dest_dir | length) == 0 {
    echo 'export PATH="$HOME/.local/bin:$PATH"' | append file ($home_dir + "/.bashrc")
    echo $"Added ($dest_dir) to PATH."
}

# Clean up
rm -rf nu $tarball

echo "Nushell installed successfully! You can now run 'nu' from your terminal."

# Set Nushell as the default shell if requested
if $env.set_default_shell == "true" {
    echo "Setting Nushell as the default shell..."
    sudo chsh -s $"($dest_dir)/nu" ($nu.env.USER)
    echo "Nushell is now your default shell."
}
