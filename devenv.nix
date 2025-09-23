{pkgs, ...}: {
  packages = with pkgs; [
    just
  ];

  enterTest = ''
    cargo --version
    just --version
  '';

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
