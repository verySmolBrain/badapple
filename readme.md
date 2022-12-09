Plays a video in the terminal as ASCII.

# Quickstart

To run with cargo:
```
cargo run -- <video-path>
```

To run with release:
```
cargo build --release
./target/release/badapple <video-path>
```

# Demo

![Demo](resources/demo.gif)

# Dependencies

Relies on opencv.

On MacOS:

```
brew install opencv
```

On Debian systems:

```
sudo apt install libopencv-dev clang libclang-dev
```