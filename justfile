debug:
    git add .
    nix build .#debug

test:
    git add .
    nix build .#test

lint:
    cargo clippy --all-targets -- -Dwarnings

[positional-arguments]
run *ARGS:
    SECOND_BRAIN_CONFIG="$PWD/data/sb_config.toml" ./result/bin/sb {{ARGS}}

