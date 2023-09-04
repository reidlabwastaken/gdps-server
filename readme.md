# gdps-server

a [Geometry Dash](https://store.steampowered.com/app/322170/Geometry_Dash/) server reimplementation in [Rust](https://rust-lang.org)

this project is based off of (stolen from) the [crystal-gauntlet](https://git.oat.zone/oat/crystal-gauntlet) server

## why?

i've run out of ideas.

### features

_these features are not yet implemented_
- utilizes `yt-dlp` and `ffmpeg` to let you upload custom songs from sources such as [YouTube](https://youtube.com), [SoundCloud](https://soundcloud.com), and [1800+ other sources](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)
- admin control panels

_these features are implemented_
- none muhuhhahaha

## build

### migrating databases

- run `cargo install diesel_cli --no-default-features --features postgres`
- run `diesel migration run`

### testing

- run `cargo run`

### building

- run `cargo build`

## todo

- probably work on the code warnings we get hehe
- <small>green name users...</small> (add udid auth to auth function, use userName instead of accountID in uploading levels, and it goes on and on and on <small>and on...</small>)
- add level parsing
- maybe swap to timestamp type instead of `(TO_CHAR(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS.MS'))` (thats REALLY ugly!!)