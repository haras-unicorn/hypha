{
  flake.lib.vcs.mkDevShell = pkgs: pkgs.mkShell {
    packages = with pkgs; [
      git
    ];
  };
}
