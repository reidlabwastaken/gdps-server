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

- User icons in account management pages
- Account settings page
- Better web design
- Use chrono for dates in database, add recent
- 2.2's friends only unlisted
- Dailies, weeklies, events(?)
- Moderation utilities
- Better song support
- Cache authentication
- Panic less
- Make a proper rank system (reuploading, uploading music, rating, etc.)
- Swap to a better web framework