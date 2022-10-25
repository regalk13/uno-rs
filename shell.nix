    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    { pkgs ? import <nixpkgs> {} }:
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = [
            # Rust related dependencies
            pkgs.rustup
            pkgs.cargo
            pkgs.rustfmt
            pkgs.libclang

        ];

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    }
