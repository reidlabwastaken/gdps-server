# gdps-server

a [Geometry Dash](https://store.steampowered.com/app/322170/Geometry_Dash/) server reimplementation in [Rust](https://rust-lang.org)

this project is based off of (stolen from) the [crystal-gauntlet](https://git.oat.zone/oat/crystal-gauntlet) server

## why?

i'm trying to learn some rust, and this is a solid choice. most GDPS solutions out there are pretty garbage, to say the least.

### features

- [highly configurable](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/config.example.toml)
- compiled, for extra speed
- [parses uploaded levels](https://git.reidlab.online/reidlab/gdps-server/src/branch/main/src/helpers/levels.rs) to patch RCEs and verify that everything is in place
- website frontend

## build

`cargo build --release`

## configuration

copy `.env.example` to `.env` and fill it out, same for `config.example.toml` to `config.toml`

## setup

make sure you have `sqlx-cli` by running `cargo install sqlx-cli`

run `sqlx database setup`

then finally run the server with `cargo run`

## CLI tools

add `--help` or `-h` when calling the binary for special tools, such as patching executables.

## todo

- user icons in account management pages
- account settings page
- better web design (make formatting more consistant)
- use chrono for dates in database
- 2.2's friends only unlisted
- dailies, weeklies, events(?)
- moderation utilities
- better song support
- cache authentication (redis or mem)
- make a proper rank system (reuploading, uploading music, rating, etc.)
- use serde to make the forms whateverCaseThisIs rather than breaking our lint convention
- add back `realip` header support
- add configurable form limits
- nix
- clean up difficulty/demon difficulties. It's fucking VILE.
- panic less, use results
- use anyhow for error handling