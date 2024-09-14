# video2ascii-rs

Yet another video2ascii in rust.

This project use [nix](https://nixos.org/download/) package manager to manage system-level dependencies,
which greatly simplifies the setup of the development environment, so you need to have nix installed first.

## How to Run

Download `BadApple.mp4` into project root directory first: <https://github.com/ryan4yin/video2chars/raw/master/tests/BadApple.mp4>

Then run the program via nix:

```shell
nix run
```

Or run without clone this repo:

```shell
nix run github:ryan4yin/video2ascii-rs#default
```

## Development

Then run the following command to build the project:

```shell
nix build
```

Or:

```shell
nix develop
cargo build
```

See [opencv-rust](https://github.com/twistedfall/opencv-rust#getting-opencv)'s docs for dependencies on other system.

## Demo

![](./badapple-demo.gif)

## Related Projects

- [video2chars(Python)](https://github.com/yuansuye/video2chars)
- [Video2ASCII.jl(Julia)](https://github.com/ryan4yin/Video2ASCII.jl)
- [Video2ASCII.jl(Go)](https://github.com/ryan4yin/Video2ascii)
- [video2ascii-c(C)](https://github.com/ryan4yin/video2ascii-c)
