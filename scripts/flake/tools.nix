{ rumor, unstableNixpkgs, ... }:

# TODO: pin dioxus-cli to the same version as in Cargo.lock

{
  flake.lib.tools.mkDevShell = pkgs:
    let
      unstablePkgs = import unstableNixpkgs {
        system = pkgs.system;
      };
    in
    pkgs.mkShell {
      packages = with pkgs; [
        unstablePkgs.dioxus-cli
        unstablePkgs.tailwindcss
      ];
    };
}
