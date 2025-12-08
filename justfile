debug: add
    nix build .

check: add
    nix flake check
    nix build .

lint:
    cargo clippy --all-targets -- -Dwarnings

add:
    git add .

[positional-arguments]
run *ARGS:
    SECOND_BRAIN_CONFIG="$PWD/examples/sb_config.toml" ./result/bin/sb {{ARGS}}

