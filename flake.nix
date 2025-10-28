{
  description = ''
    Build and devenv package for second-brain application
  '';

  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} (inputs.import-tree ./nix);

  inputs = {
    # flake inputs
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    import-tree.url = "github:vic/import-tree";

    # project inputs
    devenv.url = "github:cachix/devenv";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {nixpkgs.follows = "nixpkgs";};
    };
    nix2container = {
      url = "github:nlewo/nix2container";
      inputs = {nixpkgs.follows = "nixpkgs";};
    };
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    rust-flake.url = "github:juspay/rust-flake";
  };
}
