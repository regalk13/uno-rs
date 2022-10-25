let
    nixgl = import (fetchTarball "https://github.com/guibou/nixGL/archive/master.tar.gz") {};
    pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/5658fadedb748cb0bdbcb569a53bd6065a5704a9.tar.gz") {};
in
    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = [
            # Rust related dependencies
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.libclang

        ];

        # Point bindgen to where the clang library would be
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        # Make clang aware of a few headers (stdbool.h, wchar.h)
        BINDGEN_EXTRA_CLANG_ARGS = with pkgs; ''
          -isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include
          -isystem ${llvmPackages.libclang.out}/lib/clang/${lib.getVersion clang}/include
          -isystem ${glibc.dev}/include
        '';

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    }
