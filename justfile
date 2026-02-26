build:
    cargo build

test:
    cargo test

lint:
    cargo fmt --check
    cargo clippy --all-targets -- -Dwarnings

run *ARGS:
    SECOND_BRAIN_CONFIG="{{justfile_dir()}}/examples/sb_config.toml" \
    cargo run -- {{ARGS}}
