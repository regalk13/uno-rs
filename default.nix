let tools = import ./tools.nix; in with tools;
naersk.buildPackage {
    src = ./.;
    cargoBuild = args: '''';
    copyTarget = false;
    postInstall=''trunk build -d $out'';
    buildInputs= [trunk wasm-bindgen-cli binaryen];
}
