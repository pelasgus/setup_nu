;; manifest.scm for setup_nu
(specifications->manifest
  '("pkg-config"                ; Helper tool used when compiling
    "rust"                      ; Rust compiler
    "rust-cargo"                ; Cargo package manager
    "rust-rt-format"            ; Rust implementation of the format macro
    "rust-analyzer"
 ))
