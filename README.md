# Rust

### restclient

     nix-shell -p pkgs-config -p openssl --run zsh

### db

     nix-shell -p sqlx-cli --run zsh
     sqlx database create
     sqlx migrate add initial //initial is the name of the migration
