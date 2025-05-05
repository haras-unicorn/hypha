{ self, ... }:

{
  flake.lib.wasmBindgenCli = pkgs:
    let
      cargoLock = builtins.fromTOML
        (builtins.readFile "${self}/Cargo.lock");

      wasmBindgen = pkgs.lib.findFirst
        (pkg: pkg.name == "wasm-bindgen")
        (throw "Could not find wasm-bindgen package")
        cargoLock.package;

      wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
        version = wasmBindgen.version;
        hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
        cargoHash = "sha256-tD0OY2PounRqsRiFh8Js5nyknQ809ZcHMvCOLrvYHRE=";
      };
    in
    wasm-bindgen-cli;
}
