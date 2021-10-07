<h1 align='center'>jsort</h1>
<p align="center">
  <b>sort json, faster</b><br />
  <sub>command line tool to sort json recursively and faster</sub>
</p>
<br />

## Purpose

I needed something fast to sort object keys inside huge json files.

## Building

Make sure you have rust and cargo installed.

1. Clone the repo.

```sh
git clone https://github.com/mohitsinghs/jsort
```

2. Now compile a release binary.

```sh
cd jsort
cargo build --release
```

3. And install our binary.

```sh
sudo install ./target/release/jsort /usr/local/bin
```

## Usage

```sh
jsort <json-file(s)|(json-file-glob)>
```

For example &mdash;

```sh
jsort data.json
# or
jsort data/**/*.json
```
