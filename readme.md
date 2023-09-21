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

- run `cargo build --release`

## todo

- swap to chrono instead of `(TO_CHAR(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS.MS'))` (thats REALLY ugly!!) this would also make the `28` (upload) and `29` (update) responses work on downloadlevel and the `4` (recent) on getlevels 
- 2.2 friends only unlisted
- add more old endpoints + better support for older versions
- add dailies, events, weekly
- better way for checking if song is custom (currently `id > 50`)
- moderation utilities
- ip actions
- better song support
- return "-1" instead of panicking for stuff
- authentication caching (ip? redis?)
- idfk where to put this but i need to rant about this. why cant you have `get` and `post` in the same function for rocket. like??? why??
- use log instead of println
- find what the fuck level info is. gddocs just says "a random gzip string" like bro what
- unscuff parsing