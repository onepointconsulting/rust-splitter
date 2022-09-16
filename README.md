# Rust Splitter

Simple program used to split files using glob patterns. Files can be split by line (default mode) or
using a regular expression. 

## Compilation

```ps1
& cargo build --release
```

## Usage

```
splitter
Simple program used to split files using glob patterns. Files can be split by line (default mode) or
using a regular expression

USAGE:
    splitter.exe [OPTIONS] --pattern <PATTERN> --length-str <LENGTH_STR>

OPTIONS:
    -h, --help
            Print help information

    -l, --length-str <LENGTH_STR>
            The length of the split in terms of lines, like e.g. 50

    -p, --pattern <PATTERN>
            The pattern used to list files, like e.g. *.csv or /media/**/*.csv

    -r, --record-regex <RECORD_REGEX>
            A regular expression used to split the lines, like e.g: (?<!\\)\r?\n

    -t, --target-dir <TARGET_DIR>
            An optional target directory for all of the files. If not specified the split files will
            be in the folder of the original file
```