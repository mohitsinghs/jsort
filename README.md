# jsort <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/mohitsinghs/jsort/release?style=flat-square" /> <img alt="GitHub" src="https://img.shields.io/github/license/mohitsinghs/jsort?style=flat-square" />

Sort JSON recursiverly and faster

## Installing

Download [latest release](https://github.com/mohitsinghs/jsort/releases/latest) for your system and put it in the path.

## Usage

```sh
USAGE:
    jsort [OPTIONS] <input>

ARGS:
    <input>    files/directory to process

OPTIONS:
    -h, --help               Print help information
    -o, --out <out>          output directory ( ignores suffix )
    -r, --replace            sort and replace files
    -s, --suffix <suffix>    suffix for output files [default: sorted]
    -V, --version            Print version information
```

For example &mdash;

```sh
jsort data.json
# or
jsort data/**/*.json
```
