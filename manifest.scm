;; manifest.scm for setup_nu
(specifications->manifest
  '("rust"                      ; Rust compiler
    "cargo"                     ; Cargo package manager
    "tar"                       ; For extracting tarballs
    "coreutils"                 ; Basic utilities
    "findutils"                 ; Find command utilities
    "which")))                  ; Utility to locate binaries
