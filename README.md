# bingus

![Bingus](/src-tauri/icons/128x128@2x.png)

Create, play, and share bingo boards designed about the places you travel!

## Installation

### Prerequisites

#### Rust/Cargo

First, install `rustup` via the following command if you're on macOS, Linux, or BSD

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Windows users, install it via `scoop` through the following command:

```bash
scoop install rustup
```

#### Tauri

Next, install `tauri` using `cargo` with the following command:

```bash
cargo install create-tauri-app --locked
cargo install tauri-cli
```

#### Just

Finally, install `just` with `cargo` using the following command:

```bash
cargo install just
```

_If your package manager is not listed, follow the instructions on the [repository](https://www.github.com/casey/just)._

### Compilation

Compile the program using:

```bash
just run
```

Create a bingo board by clicking the "Get Started" button and enter the tiles you want into your bingo board.

**NOTE**: Bingo board projects must have **at least** 25 tiles to be able to make a board.

## Contributors:

The original contributors to the repository are:

- [Alex Janninck](https://www.github.com/Kerbidiah/)
- [Zayaan Saleem](https://www.github.com/zsaleem8/)
- [Nathaniel Farrell](https://www.github.com/nfarrell2/)
- [Devin Bell](https://www.github.com/devinpbell/)
