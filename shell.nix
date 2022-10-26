let tools = import ./tools.nix; in

tools.pkgs.mkShell {
    nativebuildInputs = with tools; [
      rustc
      cargo
      trunk
      wasm-bindgen-cli
      binaryen
    ];
}
