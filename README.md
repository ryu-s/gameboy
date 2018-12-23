[![Travis-CI](https://travis-ci.org/rim-buei/gameboy.svg)](https://travis-ci.org/rim-buei/gameboy)

# Game Boy
A Game Boy emulator written in Rust

![image](https://user-images.githubusercontent.com/43806767/48777725-b82fbd80-ed16-11e8-8ab9-82351dfe0f8c.png)

# Project Status
- Still work in progress

# Requirements
- Rust
- cargo-web

## Setup Rust
```sh
$ curl https://sh.rustup.rs -sSf | sh
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown
```

## Setup `cargo-web`
```sh
$ cargo install cargo-web
```

# How to Run
Clone this repo and launch `cargo-web`.
```sh
$ git clone https://github.com/rim-buei/gameboy.git
$ cd gameboy
$ cargo web start --bin wasm --target wasm32-unknown-unknown
```

Then, browse `http://localhost:8000`.

# Emulation Accuracy
Currently, this emulator passes [the Blargg's](http://gbdev.gg8.se/files/roms/blargg-gb-tests/) CPU instruction test cases.

![image](https://user-images.githubusercontent.com/43806767/50381598-5aecaa80-06ce-11e9-8415-6df6c5a5e1fe.png)
