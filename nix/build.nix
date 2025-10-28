{inputs, ...}: {
  imports = with inputs.rust-flake; [
    flakeModules.default
    flakeModules.nixpkgs
  ];
  perSystem = {self', ...}: {
    packages.default = self'.packages.second-brain;
  };
}
