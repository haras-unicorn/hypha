{ self, naersk, lib, rust-overlay, ... }:

let
  mkNaerskLib = pkgs: pkgs.callPackage naersk {
    cargo = pkgs.rust-naersk;
    rustc = pkgs.rust-naersk;
  };

  nativeBuildInputs = pkgs: [
    pkgs.pkg-config
    pkgs.lld
  ];

  buildInputs = pkgs: [
    pkgs.openssl
    pkgs.libiconv
    pkgs.pkg-config
  ] ++ lib.optionals
    pkgs.stdenv.isLinux
    [
      pkgs.glib
      pkgs.gtk3
      pkgs.libsoup_3
      pkgs.webkitgtk_4_1
      pkgs.xdotool
    ] ++ lib.optionals
    pkgs.stdenv.isDarwin
    (with pkgs.darwin.apple_sdk.frameworks; [
      IOKit
      Carbon
      WebKit
      Security
      Cocoa
    ]);
in
{
  flake.lib.rust.overlays = [
    (import rust-overlay)
    (final: prev: {
      rust-naersk = prev.rust-bin.stable.latest.default.override {
        extensions = [
          "clippy"
          "rustfmt"
          "rust-analyzer"
          "rust-src"
        ];
        targets = [
          "wasm32-unknown-unknown"
        ];
      };
    })
  ];

  flake.lib.rust.mkPackage = pkgs:
    let
      naerskLib = mkNaerskLib pkgs;
    in
    naerskLib.buildPackage {
      name = "hypha";
      pname = "hypha";
      version = "0.1.0";
      src = self;

      nativeBuildInputs = nativeBuildInputs pkgs;

      buildInputs = buildInputs pkgs;

      cargoBuildOptions = prev: prev
        ++ [ "--features" "desktop" ];
    };

  flake.lib.rust.mkDevShell = pkgs:
    pkgs.mkShell {
      shellHook = ''
        export RUST_BACKTRACE="full";
      '';

      nativeBuildInputs = nativeBuildInputs pkgs;

      buildInputs = buildInputs pkgs;

      packages = with pkgs; [
        llvmPackages.clangNoLibcxx
        lldb
        rust-naersk
        cargo-edit
        cargo-expand
        evcxr
        (self.lib.wasmBindgenCli pkgs)
      ];
    };
}

