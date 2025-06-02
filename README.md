<!-- badges: start -->

[![GitHub license](https://img.shields.io/github/license/Y4dd/bang-cli)](https://github.com/Y4dd/bang-cli/blob/master/LICENSE)

<!-- badges: end -->

# Bang CLI

A blazingly fast CLI made in rust that compiles DuckDuckGo-style `!bang` shortcuts into a binary map for instant query resolution

## Disclaimer

This queries DuckDuckGo's bang data and saves it as binary in the user's data directory
You can view the JSON data at [DuckDuckGo BANG!](https://duckduckgo.com/bang.js)

## Installation

```bash
cargo install bang-cli
```

or local build

```bash
git clone git@github.com:Y4dd/bang-cli.git
cd bang-cli
cargo install --path .
```

## Usage

### Query

```bash
# bang ![tag] [query]
bang !npm typescript
# https://www.npmjs.com/search?q=typescript
bang !npm
# https://www.npmjs.com

# You can search bangs on their official website
bang !bangs cargo
```

### Utility

The CLI downloads and saves the data at first run if not found.
If for any reason you want to delete it or rebuild it

```bash
# Deletes from data directory
bang --clean
# Deletes, fetches and rebuilds into data directory
bang --rebuild
```

### Usage with Linux launchers

I've provided simple entry scripts under the `./scripts` for launchers i've tested.
Personally, i use wofi in both drun and dmenu mode as seen in `./scripts/bang-wrapper.sh`

## License

This project is licensed under the [MIT License](./LICENSE).
