{inputs, ...}: {
  imports = with inputs; [devenv.flakeModule];

  perSystem = {pkgs, ...}: {
    devenv.shells.default = {
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
    };
  };
}
