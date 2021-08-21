# video2ascii-rs

yet another video2ascii in rust.


## Dependencies

download `BadApple.mp4` into project root directory first: <https://github.com/ryan4yin/video2chars/raw/master/tests/BadApple.mp4>

then install dependencies, `opensuse` as an example:

```shell
sudo zypper in libvpx-devel libopus-devel
sudo zypper in opencv opencv-devel
sudo zypper in clang clang-devel
# use mpv to play audio
sudo zypper in mpv
```

see [opencv-rust](https://github.com/twistedfall/opencv-rust#getting-opencv)'s docs for dependencies on other system.

## How to Run

run the program via `cargo`:

```shell
cargo run
```
