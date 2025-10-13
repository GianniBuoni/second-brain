debug:
    git add .
    nix build .

test:
    git add .
    nix build .
    nix flake check --all-systems .

lint:
    cargo clippy --all-targets -- -Dwarnings

[positional-arguments]
run *ARGS:
    SECOND_BRAIN_CONFIG="$PWD/examples/sb_config.toml" ./result/bin/sb {{ARGS}}

