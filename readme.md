# gdps-server

a [Geometry Dash](https://store.steampowered.com/app/322170/Geometry_Dash/) server reimplementation in [Rust](https://rust-lang.org)

this project is based off of (stolen from) the [crystal-gauntlet](https://git.oat.zone/oat/crystal-gauntlet) server

## why?

i've run out of ideas.

### features

- [highly configurable](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/config.example.toml)
- compiled, for extra speed
- [parses uploaded levels](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/src/helpers/levels.rs) to patch RCEs and verify that everything is in place

## build

### migrating databases

- run `cargo install diesel_cli --no-default-features --features postgres`
- run `diesel migration run`

### testing

- run `cargo run`

### building

- run `cargo build --release`

## todo

- chrono
- 2.2 friends only unlisted
- add dailies, events, weekly
- moderation utilities
- ip actions
- better song support
- authentication caching (ip? redis?)
- use log instead of println