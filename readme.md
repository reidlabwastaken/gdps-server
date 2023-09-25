# gdps-server

a [Geometry Dash](https://store.steampowered.com/app/322170/Geometry_Dash/) server reimplementation in [Rust](https://rust-lang.org)

this project is based off of (stolen from) the [crystal-gauntlet](https://git.oat.zone/oat/crystal-gauntlet) server

## why?

i've run out of ideas.

### features

- [highly configurable](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/config.example.toml)
- compiled, for extra speed
- [parses uploaded levels](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/src/helpers/levels.rs) to patch RCEs and verify that everything is in place
- website frontend

## build

### migrating databases

- run `cargo install diesel_cli --no-default-features --features postgres`
- run `diesel migration run`

### testing

- run `cargo run`

### building

- run `cargo build --release`

## todo

- account settings page
- better web design
- use chrono
- 2.2 friends only unlisted
- add dailies, events, weekly
- moderation utilities
- better song support
- authentication caching
- use log instead of println
- make a proper rank system (reuploading, uploading music, rating, etc.)
- user icons in the account management + settings (gdicon.oat.zone? selfhost?) ideally we find a legal way to do this (i cant distribute the plist+asset files directly) but doing this illegally is always an option