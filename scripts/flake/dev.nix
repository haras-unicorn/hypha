{ pkgs
, self
, ...
}:

{
  seal.defaults.devShell = "dev";
  integrate.devShell = {
    nixpkgs.overlays = self.lib.rust.overlays;
    devShell = pkgs.mkShell {
      inputsFrom = [
        (self.lib.vcs.mkDevShell pkgs)
        (self.lib.scripts.mkDevShell pkgs)
        (self.lib.rust.mkDevShell pkgs)
        (self.lib.format.mkDevShell pkgs)
        (self.lib.lint.mkDevShell pkgs)
        (self.lib.tools.mkDevShell pkgs)
        (self.lib.lsp.mkDevShell pkgs)
      ];
    };
  };
}
