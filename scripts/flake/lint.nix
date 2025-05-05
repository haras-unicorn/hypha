{ self, ... }:

{
  flake.lib.lint.mkDevShell = pkgs: pkgs.mkShell {
    inputsFrom = [
      (self.lib.rust.mkDevShell pkgs)
    ];
    packages = with pkgs; [
      nodePackages.prettier
      nodePackages.cspell
      just
      nushell
      nixpkgs-fmt
      markdownlint-cli
      nodePackages.markdown-link-check
    ];
  };
}
