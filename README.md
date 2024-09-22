# Action `setup_nu`
## 📝｜ABSTRACT
This GitHub Action will setup a Nushell environment. 

### FEATURES
- Automatically detects system architecture and OS to download the correct Nushell version.
- Installs Nushell.
- Optionally sets Nushell as the default shell.

## 🧰｜USAGE
```
name: Test Nushell Setup

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  setup-nushell:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Code
        uses: actions/checkout@v4

      - name: Install Nushell
        uses: pelasgus/setup_nu@main
```
**N.B.** : To make `nushell` the default shell add:
```
with:
  set-default: true
```

## 🌱｜Local Development
### 🚨｜GENERAL PREREQUISITES
```nu
guix shell -m manifest.scm
```
```nu
cargo build --release
```

## ⚖️｜LICENSE
The repository's contents are licensed under the latest version of the [GNU General Public License](https://www.gnu.org/licenses/gpl-3.0.html).
