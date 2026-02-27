{inputs, ...}: {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem = {pkgs, ...}: {
    devshells.default = {extraModulesPath, ...}: {
      imports = [
        "${extraModulesPath}/language/rust.nix"
        "${extraModulesPath}/language/c.nix"
      ];

      packages = with pkgs; [
        just
        rust-analyzer
      ];

      commands = [
        {
          name = "enterTest";
          help = "Test dev shell has all required tooling.";
          command = ''
            cargo -V
            cargo clippy -V
            just -V
          '';
        }
      ];
    };
  };
}
