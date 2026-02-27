let
  pname = "sb";
in {
  perSystem = {
    self',
    pkgs,
    ...
  }: {
    packages = {
      ${pname} = pkgs.rustPlatform.buildRustPackage {
        inherit pname;
        version = "0.3.0";
        cargoLock.lockFile = ../Cargo.lock;
        src = pkgs.lib.cleanSource ../.;
      };
      default = self'.packages.${pname};
    };
  };
}
