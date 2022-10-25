let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

  with nixpkgs;

  mkShell {
    buildInputs = [
      clang # needed for bindgen
      latest.rustChannels.nightly.rust
      openssl
      pkgconfig # needed for libdbus-sys to find the paths
      dbus.dev
    ];

    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  }

