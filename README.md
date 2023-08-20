# hebi

tiny hydra renderer written in Rust.

It is a renderer that simply fills the screen with green.

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">HoudiniのSolarisのHydraレンダラーを自作の緑単色塗りつぶしHydraレンダラーに置き換えることができた～！ 非常に小さなHydraレンダラーということでhebiという名前のレンダラーにしました。 <a href="https://t.co/VabcggGAL0">pic.twitter.com/VabcggGAL0</a></p>&mdash; ᗦ)))⋊ - オリトイツキ - ᗦ+++◃ (@MatchaChoco010) <a href="https://twitter.com/MatchaChoco010/status/1672585055468945408?ref_src=twsrc%5Etfw">June 24, 2023</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
## Dependencies

- MSVC 2019 or later
- CMake 3.24 or later
- Python 3.x
- Rust 1.70 or later
- [cargo-make](https://github.com/sagiegurari/cargo-make)

The current build script only supports windows, USD v22.05 and Houdini.

## How to build

```sh
$ cargo make
```

The first time build will take several minutes.
