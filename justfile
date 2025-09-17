debug:
    git add .
    nix build .#debug

test:
    git add .
    nix build .#test
