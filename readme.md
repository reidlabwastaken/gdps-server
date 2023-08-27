# gdps-server

a [Geometry Dash](https://store.steampowered.com/app/322170/Geometry_Dash/) server reimplementation in [Rust](https://rust-lang.org), focusing on 1:1 recreations of vanilla GD features

_this project is in early stages. it is NOT production ready._

_ONLY 2.2 is supported._

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

- run `cargo run run`

### building

- run `cargo build`

## todo

- add login endpoint....... NOW!
- our passwords are a little insecure (`argon2(sha1(password + "mI29fmAnxgTs"))`) and there isnt anything we can do about this because gpj2 is forced like that!! thanks robtop!! (try and find a fix anyway lul)