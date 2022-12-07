# Vocab Helper

An application for learning vocabulary sets (or it should become that eventually).

<br>

> This is (at least based on) a school project within the topic 'databases'. <br>
> It is not to be taken seriously in any way.

<br>

## Content
- [Technical Details](#Details "Goto section 'Details'")
- [Progress](#Progress "Goto section 'Progress'")
- [Getting Started](#GettingStarted "Goto section 'Getting Started'")
  - [Build Instructions](#Build "Goto section 'Build'")
  - [Documentation](#Docs "Goto section 'Docs'")

<br>

## Details

**Built using the [Rust Programming Language](https://www.rust-lang.org/)**

> Sqlite: [rusqlite](https://github.com/rusqlite/rusqlite) (wrapper for the sqlite database engine) <br>
> Frontend: [macroquad](https://github.com/not-fl3/macroquad) (game engine with intermediate mode ui and easy cross compilation)

<br>

## Progress

- [ ] Planned functionality:
  - [X] Store Vocabulary Pairs
  - [X] Recall them
  - [ ] GUI
  - [ ] Flashcard mode (iterate through pair, switch between word - translation)
  - [ ] Quiz mode (typing)
  - [ ] GUI
  - [ ] Settings

- Planned features:
  - Unicode support (should already work..)

<br>

## Getting Started

### Build

> It's Rust, so just [make sure Rust is installed on your system](https://www.rust-lang.org/tools/install "don't forget to run `rustup install stable`") and run
```sh
cargo run
```
> in the [directory above the crate root](https://rust-lang.org/ "In the top most directory, where you find the Cargo.toml file"). <br>
> To cross compile for android run the appropriate script (you find it in the same directory)
```sh
./build_for_android.sh
```

