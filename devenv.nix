{pkgs, ...}: {
  packages = with pkgs; [
    just
  ];

  enterShell = ''export SECOND_BRAIN_CONFIG="$PWD/data/sb_config.toml"'';

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
