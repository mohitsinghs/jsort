<h1 align='center'>sort-of</h1>
<p align="center">
  <b>sort json, faster</b><br />
  <sub>command line tool to sort json recursively and faster</sub>
</p>
<br />

## Purpose

I needed something fast to sort object keys inside huge json files, so I put together this quickly. That job is done and now this dude lives in my system waiting for it's chance.

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

3. And install our binary.

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
