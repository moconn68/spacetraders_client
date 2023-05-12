# SpaceTraders Client: An Elegant CLI 🪐

## About ℹ️

This project is a CLI client for the [SpaceTraders API](https://spacetraders.io/), which is "a unique multiplayer game
built on a free Web API."

## Features 🔔

- Simple command line interface for interacting with the game's API
- Powerful game automation at your fingertips
- Elegant resource management for you and your fleet

## Usage 🛠️

### Pre-requisites 💾

This project doesn't yet have any releases, so to use it you must build from source. To do so, you must first install the Rust programming language and its toolchain. The standard method for this is to use [rustup](https://www.rust-lang.org/tools/install):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You'll also need an access token for the SpaceTraders API, as well as to have already registered a new agent (doing so from the client directly coming soon). To do this, follow the first two sections of the [New Game Quickstart docs](https://docs.spacetraders.io/quickstart/new-game).

### Installation ⚙️

1. Clone this project
2. In the root of the project, create a new file named `.env`. Open this file and paste in your API token. Don't forget to save!
3. Run the client with `cargo run` from the root directory

## Software used 👨‍💻

- rustlang
- reqwests
- serde
