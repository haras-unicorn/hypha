{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    unstableNixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    perch.url = "github:altibiz/perch/refs/tags/2.1.1";
    perch.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { perch, ... } @inputs:
    perch.lib.flake.make {
      inherit inputs;
      root = ./.;
      prefix = "scripts/flake";
    };
}
