{ self, ... }:

{
  flake.lib.format.mkDevShell = pkgs: pkgs.mkShell {
    inputsFrom = [
      (self.lib.rust.mkDevShell pkgs)
    ];
    packages = with pkgs; [
      nodePackages.prettier
      just
      nushell
      nixpkgs-fmt
    ];
  };
}
