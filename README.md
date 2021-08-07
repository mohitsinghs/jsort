<h1 align='center'>sort-of</h1>
<p align="center">
  <b>sort json, faster</b><br />
  <sub>command line tool to sort json recursively and faster</sub>
</p>
<br />

## Purpose

I needed something to sort object keys quickly inside huge json files, so I put together this quickly.

## Building

Make sure you have rust and cargo installed.

1. Clone the repo.

```sh
git clone https://github.com/mohitsinghs/sort-of
```

2. Now compile a release binary.

```sh
cd sort-of
cargo build --release
```

3. Add install that to system.

```sh
sudo install ./target/release/sort-of /usr/local/bin
```

## Usage

```sh
sort-of <json-file(s)|(json-file-glob)>
```

For example &mdash;

```sh
sort-of data.json
# or
sort-of data/**/*.json
```
