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
- Flesh out account management page
- Account settings page
- Better web design (make formatting more consistant)
- Use chrono for dates in database, add recent
- 2.2's friends only unlisted
- Dailies, weeklies, events(?)
- Moderation utilities
- Better song support
- Cache authentication
- Panic less
- Make a proper rank system (reuploading, uploading music, rating, etc.)
- Use serde to make the forms whateverCaseThisIs rather than breaking our lint convention
- Swap to `sqlx` im gonna be honest `diesel` is pretty shit.
- Swap to `sqlite` from `postgres`. Postgres feels too clunky and it just solos honestly
- Add back `realip` header support
- Add configurable form limits