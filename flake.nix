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
    rust-flake.url = "github:juspay/rust-flake";
  };
}
